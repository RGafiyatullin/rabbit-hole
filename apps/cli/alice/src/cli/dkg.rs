use structopt::StructOpt;

use crate::AnyError;

use super::Cli;

mod csi_rashi;

#[derive(Debug, StructOpt)]
pub struct Dkg {
    #[structopt(subcommand)]
    cmd: Cmd,
}

#[derive(Debug, StructOpt)]
enum Cmd {
    /// CSI-RAShi (https://ia.cr/2020/1323)
    CsiRashi(csi_rashi::CsiRashi),
}

impl Dkg {
    pub async fn run(&self, cli: &Cli) -> Result<(), AnyError> {
        match &self.cmd {
            Cmd::CsiRashi(sub) => sub.run(self, cli).await,
        }
    }
}
