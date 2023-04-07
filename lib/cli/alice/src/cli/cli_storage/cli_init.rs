use structopt::StructOpt;

use crate::cli::{Cli, CliRun};
use crate::AnyError;

use super::CliStorage;

#[derive(Debug, StructOpt)]
pub struct CliInit {}

impl<'a, F, G, H> CliRun<(&'a CliStorage, &'a Cli<F, G, H>)> for CliInit {
    fn run(&self, (_cmd_storage, cli): (&'a CliStorage, &'a Cli<F, G, H>)) -> Result<(), AnyError> {
        let storage = cli.open_storage()?;
        storage.flush()?;
        
        Ok(())
    }
}
