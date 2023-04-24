use rand::RngCore;
use structopt::StructOpt;

use cli_storage::Storage;

use crate::caps::IO;
use crate::cli::CliRun;
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

impl<R: RngCore, I: IO> CliRun<(R, I, Storage)> for CmdDkg {
    fn run(&self, (rng, io, storage): (R, I, Storage)) -> Result<RetCode, AnyError> {
        match &self.cmd {
            Cmd::CsiRashi(sub) => sub.run((rng, io, storage)),
        }
    }
}
