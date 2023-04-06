use std::str::FromStr;

#[derive(Debug, Clone, structopt::StructOpt)]
pub struct LoggingConfig {
    #[structopt(long, default_value = "info")]
    pub min_log_level: tracing::Level,

    #[structopt(long)]
    pub log_target_filter: Vec<LogTargetConfig>,
}

#[derive(Debug, Clone)]
pub struct LogTargetConfig {
    pub path: Vec<String>,
    pub level: tracing::Level,
}

impl FromStr for LogTargetConfig {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (path, level) = s.split_once('=').ok_or_else(|| "eq-sign missing".to_owned())?;
        let level = tracing::Level::from_str(level).map_err(|e| e.to_string())?;
        let path = path.split("::").map(|s| s.to_owned()).collect();

        let out = Self { path, level };
        Ok(out)
    }
}
