use structopt::StructOpt;

use crate::AnyError;

use super::Cli;

mod dkls;
mod frost;

#[derive(Debug, StructOpt)]
pub struct Tss {
    #[structopt(subcommand)]
    cmd: Cmd,
}

#[derive(Debug, StructOpt)]
enum Cmd {
    /// FROST (https://ia.cr/2020/852)
    Frost(frost::Frost),
    /// DKLS (https://ia.cr/2018/499)
    Dkls(dkls::Dkls),
}

impl Tss {
    pub async fn run(&self, cli: &Cli) -> Result<(), AnyError> {
        match &self.cmd {
            Cmd::Frost(sub) => sub.run(self, cli).await,
            Cmd::Dkls(sub) => sub.run(self, cli).await,
        }
    }
}
