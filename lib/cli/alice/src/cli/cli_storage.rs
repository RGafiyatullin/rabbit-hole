use structopt::StructOpt;

use crate::AnyError;

use super::{Cli, CliRun};

mod cli_init;

#[derive(Debug, StructOpt)]
pub struct CliStorage {
    #[structopt(subcommand)]
    cmd: Cmd,
}

#[derive(Debug, StructOpt)]
enum Cmd {
    Init(cli_init::CliInit),
}

impl<F, G, H> CliRun<&Cli<F, G, H>> for CliStorage {
    fn run(&self, cli: &Cli<F, G, H>) -> Result<(), AnyError> {
        match &self.cmd {
            Cmd::Init(sub) => sub.run((self, cli)),
        }
    }
}
