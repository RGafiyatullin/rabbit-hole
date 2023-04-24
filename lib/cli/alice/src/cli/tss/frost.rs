use rand::RngCore;
use structopt::StructOpt;

use cli_storage::Storage;

use crate::caps::IO;

#[derive(Debug, StructOpt)]
pub struct CmdFrost {
    #[structopt(long, short)]
    key_id: String,

    #[structopt(subcommand)]
    cmd: Cmd,
}

#[derive(Debug, StructOpt)]
enum Cmd {
    Prepare,
    Sign,
    Aggregate,
}

pub fn run(
    dkls: &CmdFrost,
    rng: impl RngCore,
    io: impl IO,
    storage: Storage,
) -> Result<crate::RetCode, crate::AnyError> {
    unimplemented!()
}
