use rand::RngCore;
use structopt::StructOpt;

use cli_storage::Storage;

use crate::caps::IO;
use crate::cli::RetCode;
use crate::AnyError;

#[derive(Debug, StructOpt)]
pub struct CmdDkls {}

pub fn run(
    dkls: &CmdDkls,
    rng: impl RngCore,
    io: impl IO,
    storage: Storage,
) -> Result<RetCode, AnyError> {
    unimplemented!()
}
