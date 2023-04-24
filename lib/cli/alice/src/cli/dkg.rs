use rand::RngCore;
use structopt::StructOpt;

use cli_storage::Storage;

use crate::caps::IO;
use crate::{AnyError, RetCode};

mod csi_rashi;

#[derive(Debug, StructOpt)]
pub struct CmdDkg {
    #[structopt(subcommand)]
    cmd: Cmd,
}

#[derive(Debug, StructOpt)]
enum Cmd {
    CsiRashi(csi_rashi::CmdCsiRashi),
}

pub fn run(
    dkg: &CmdDkg,
    rng: impl RngCore,
    io: impl IO,
    storage: Storage,
) -> Result<RetCode, AnyError> {
    match &dkg.cmd {
        Cmd::CsiRashi(sub) => csi_rashi::run(sub, rng, io, storage),
    }
}
