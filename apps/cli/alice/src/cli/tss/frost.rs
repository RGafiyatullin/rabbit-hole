use structopt::StructOpt;

use crate::curve::Curve;
use crate::AnyError;

use super::{Cli, Tss};

mod aggregate;
mod nonce;
mod sign;

#[derive(Debug, StructOpt)]
pub struct Frost {
    #[structopt(long, short)]
    curve: Curve,

    #[structopt(subcommand)]
    cmd: Cmd,
}

#[derive(Debug, StructOpt)]
enum Cmd {
    Nonce(nonce::Nonce),
    Sign(sign::Sign),
    Aggregate(aggregate::Aggregate),
}

impl Frost {
    pub async fn run(&self, tss: &Tss, cli: &Cli) -> Result<(), AnyError> {
        match &self.cmd {
            Cmd::Nonce(sub) => sub.run(self, tss, cli).await,
            Cmd::Sign(sub) => sub.run(self, tss, cli).await,
            Cmd::Aggregate(sub) => sub.run(self, tss, cli).await,
        }
    }
}
