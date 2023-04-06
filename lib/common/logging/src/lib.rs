mod config;
pub use config::{LogTargetConfig, LoggingConfig};

mod level_filter_trie;

pub fn init(config: &LoggingConfig) {
    use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;
    use tracing_subscriber::{filter, Layer};

    let filter = level_filter_trie::FilterTrie::from_statements(&config.log_target_filter);

    let fmt = tracing_subscriber::fmt::layer()
        .pretty()
        .with_target(true)
        .with_thread_names(false)
        .with_file(true)
        .with_filter(filter::LevelFilter::from_level(config.min_log_level))
        .with_filter(filter::filter_fn(move |entry| {
            filter
                .level_for_target(entry.target().split("::"))
                .map(|level| level >= *entry.level())
                .unwrap_or(false)
        }));

    tracing_subscriber::registry().with(fmt).init();
}
