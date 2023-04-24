use rand::RngCore;
use structopt::StructOpt;

use cli_storage::Storage;

use crate::caps::IO;

// mod dkls;
mod frost;

#[derive(Debug, StructOpt)]
pub struct CmdTss {
    #[structopt(subcommand)]
    cmd: Cmd,
}

#[derive(Debug, StructOpt)]
enum Cmd {
    // Dkls(dkls::CmdDkls),
    Frost(frost::CmdFrost),
}

pub fn run(
    tss: &CmdTss,
    rng: impl RngCore,
    io: impl IO,
    storage: Storage,
) -> Result<crate::RetCode, crate::AnyError> {
    match &tss.cmd {
        // Cmd::Dkls(sub) => dkls::run(sub, rng, io, storage),
        Cmd::Frost(sub) => frost::run(sub, rng, io, storage),
    }
}
