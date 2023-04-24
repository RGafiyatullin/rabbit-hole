use std::ffi::OsString;
use std::path::PathBuf;

use cli_storage::Storage;
use rand::RngCore;
use structopt::StructOpt;

use crate::caps::IO;
use crate::{AnyError, RetCode};

mod dkg;
mod keys;
mod tss;
mod verify;

#[derive(Debug, StructOpt)]
pub struct Cli {
    #[structopt(long, short, env = "ALICE_STORAGE")]
    storage_path: Option<PathBuf>,

    #[structopt(subcommand)]
    cmd: Sub,
}

#[derive(Debug, StructOpt)]
enum Sub {
    Keys(keys::CmdKeys),
    Dkg(dkg::CmdDkg),
    Tss(tss::CmdTss),
    Verify(verify::CmdVerify),
}

impl Cli {
    pub fn create(args: impl IntoIterator<Item = impl Into<OsString> + Clone>) -> Self {
        <Self as StructOpt>::from_iter(args)
    }
    pub fn create_safe(
        args: impl IntoIterator<Item = impl Into<OsString> + Clone>,
    ) -> Result<Self, AnyError> {
        <Self as StructOpt>::from_iter_safe(args).map_err(Into::into)
    }
}

pub fn run<R, I>(cli: &Cli, rng: R, io: I) -> Result<RetCode, AnyError>
where
    R: RngCore,
    I: IO,
{
    let open_storage =
        || Ok::<_, AnyError>(Storage::open(cli.storage_path()?.to_str().ok_or("invalid path")?)?);

    match &cli.cmd {
        Sub::Keys(sub) => keys::run(sub, io, open_storage()?),
        Sub::Dkg(sub) => dkg::run(sub, rng, io, open_storage()?),
        Sub::Tss(sub) => tss::run(sub, rng, io, open_storage()?),
        Sub::Verify(sub) => verify::run(sub, io),
    }
}

impl Cli {
    fn storage_path(&self) -> Result<PathBuf, AnyError> {
        if let Some(path) = self.storage_path.as_ref() {
            Ok(path.to_owned())
        } else if let Ok(path) = std::env::var("HOME") {
            let path = path.parse::<PathBuf>()?;
            Ok(path.join(".alice"))
        } else {
            Err("Failed to determine the storage-path".into())
        }
    }
}
