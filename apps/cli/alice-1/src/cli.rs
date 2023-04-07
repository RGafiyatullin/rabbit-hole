use std::path::PathBuf;

use structopt::StructOpt;

use crate::logging::LogTargetFilter;
use crate::namespace::Namespace;
use crate::AnyError;

mod capabilities;

mod dkg;
mod key_share;
mod tss;

#[derive(Debug, StructOpt)]
pub struct Cli {
    #[structopt(long, default_value = "info")]
    min_log_level: tracing::Level,

    #[structopt(long)]
    log_target_filter: Vec<LogTargetFilter>,

    #[structopt(long, short = "k")]
    secrets_key_file: Option<PathBuf>,

    #[structopt(long, short = "d")]
    secrets_data_file: Option<PathBuf>,

    #[structopt(skip)]
    secrets_ns: Namespace,

    #[structopt(subcommand)]
    cmd: Cmd,
}

#[derive(Debug, StructOpt)]
enum Cmd {
    /// Distributed Key Generation
    Dkg(dkg::Dkg),
    /// Threshold Signature Scheme
    Tss(tss::Tss),
    ///
    KeyShare(key_share::KeyShare),
}

impl Cli {
    pub async fn run(&self) -> Result<(), AnyError> {
        let _ = dotenv::dotenv();
        crate::logging::init(self.min_log_level, &self.log_target_filter);

        match &self.cmd {
            Cmd::Dkg(sub) => sub.run(self).await,
            Cmd::Tss(sub) => sub.run(self).await,
            Cmd::KeyShare(sub) => sub.run(self).await,
        }
    }
}
