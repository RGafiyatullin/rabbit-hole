use structopt::StructOpt;

use crate::AnyError;

use super::{Cli, Frost, Tss};

mod count;
mod generate;
mod list;
mod marked_used;

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
    MarkUsed(marked_used::MarkUsed),
}

impl Nonce {
    pub async fn run(&self, frost: &Frost, tss: &Tss, cli: &Cli) -> Result<(), AnyError> {
        match &self.cmd {
            Cmd::Generate(sub) => sub.run(self, frost, tss, cli).await,
            Cmd::Count(sub) => sub.run(self, frost, tss, cli).await,
            Cmd::List(sub) => sub.run(self, frost, tss, cli).await,
            Cmd::MarkUsed(sub) => sub.run(self, frost, tss, cli).await,
        }
    }
}
