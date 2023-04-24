use rand::RngCore;
use structopt::StructOpt;

use cli_storage::Storage;

use crate::caps::IO;
use crate::cli::RetCode;
use crate::AnyError;

mod aggregator;
mod cosigner;

#[derive(Debug, StructOpt)]
pub struct CmdDkls {}

pub fn run(
    _dkls: &CmdDkls,
    _rng: impl RngCore,
    _io: impl IO,
    _storage: Storage,
) -> Result<RetCode, AnyError> {
    unimplemented!()
}
