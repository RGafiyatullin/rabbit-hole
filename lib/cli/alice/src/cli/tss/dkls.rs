use rand::RngCore;
use structopt::StructOpt;

use cli_storage::Storage;

use crate::caps::IO;

#[derive(Debug, StructOpt)]
pub struct CmdDkls {}

pub fn run(
    dkls: &CmdDkls,
    rng: impl RngCore,
    io: impl IO,
    storage: Storage,
) -> Result<crate::RetCode, crate::AnyError> {
    unimplemented!()
}
