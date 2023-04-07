use std::collections::HashMap;

use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{filter, Layer};

#[derive(Debug, Clone, structopt::StructOpt)]
pub struct LoggingConfig {
    #[structopt(long, default_value = "info")]
    pub min_log_level: tracing::Level,

    #[structopt(long)]
    pub log_target_filter: Vec<LogTargetFilter>,
}

#[derive(Debug, Clone)]
pub struct LogTargetFilter {
    pub path: Vec<String>,
    pub level: tracing::Level,
}

pub fn init(min_log_level: tracing::Level, log_target_filter: &[LogTargetFilter]) {
    let filter = FilterNode::from_statements(log_target_filter);

    let fmt = tracing_subscriber::fmt::layer()
        .pretty()
        .with_target(true)
        .with_thread_names(false)
        .with_file(true)
        .with_filter(filter::LevelFilter::from_level(min_log_level))
        .with_filter(filter::filter_fn(move |entry| {
            filter
                .level_for_target(entry.target().split("::"))
                .map(|level| level >= *entry.level())
                .unwrap_or(false)
        }));

    tracing_subscriber::registry().with(fmt).init();
}

#[derive(Debug, Default)]
struct FilterNode {
    level: Option<tracing::Level>,
    children: HashMap<String, Self>,
}
impl FilterNode {
    fn level_for_target<'a>(
        &self,
        path: impl IntoIterator<Item = &'a str>,
    ) -> Option<tracing::Level> {
        let mut path = path.into_iter();
        if let Some(next) = path.next() {
            self.children
                .get(next)
                .and_then(move |node| node.level_for_target(path))
                .or_else(|| self.children.get("*").and_then(|node| node.level_for_target([])))
        } else {
            self.level
        }
    }

    fn from_statements(statements: &[LogTargetFilter]) -> Self {
        let mut root = Self::default();

        for statement in statements {
            let mut n = &mut root;
            for mod_name in statement.path.iter() {
                n = n.children.entry(mod_name.to_owned()).or_default();
            }
            n.level = Some(statement.level);
        }
        root
    }
}

impl std::str::FromStr for LogTargetFilter {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (path, level) = s.split_once('=').ok_or_else(|| "eq-sign missing".to_owned())?;
        let level = tracing::Level::from_str(level).map_err(|e| e.to_string())?;
        let path = path.split("::").map(|s| s.to_owned()).collect();

        let out = Self { path, level };
        Ok(out)
    }
}
