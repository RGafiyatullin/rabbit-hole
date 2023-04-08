use structopt::StructOpt;

use crate::AnyError;

use super::{Cli, CliRun};

#[derive(Debug, StructOpt)]
pub struct CliKeys {
    // #[structopt(subcommand)]
    // cmd: Cmd,
}

// #[derive(Debug, StructOpt)]
// enum Cmd {
// }

impl<F, G, H> CliRun<&Cli<F, G, H>> for CliKeys {
    fn run(&self, cli: &Cli<F, G, H>) -> Result<(), AnyError> {
        unimplemented!()
    }
}
