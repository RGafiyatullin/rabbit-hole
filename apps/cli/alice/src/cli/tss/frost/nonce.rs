use structopt::StructOpt;

use crate::AnyError;

use super::{Cli, Frost, Tss};

mod count;
mod generate;
mod list;
mod remove;

#[derive(Debug, StructOpt)]
pub struct Nonce {
    #[structopt(subcommand)]
    cmd: Cmd,
}

#[derive(Debug, StructOpt)]
enum Cmd {
    Generate(generate::Generate),
    Count(count::Count),
    List(list::List),
    Remove(remove::Remove),
}

impl Nonce {
    pub async fn run(&self, frost: &Frost, tss: &Tss, cli: &Cli) -> Result<(), AnyError> {
        match &self.cmd {
            Cmd::Generate(sub) => sub.run(self, frost, tss, cli).await,
            Cmd::Count(sub) => sub.run(self, frost, tss, cli).await,
            Cmd::List(sub) => sub.run(self, frost, tss, cli).await,
            Cmd::Remove(sub) => sub.run(self, frost, tss, cli).await,
        }
    }
}
