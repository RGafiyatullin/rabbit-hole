use std::ffi::OsString;
use std::path::PathBuf;

use cli_storage::Storage;
use structopt::StructOpt;

use crate::caps::IO;
use crate::{AnyError, RetCode};

mod keys;

pub trait CliRun<Prev> {
    fn run(&self, prev: Prev) -> Result<RetCode, AnyError>;
}

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

impl<I> CliRun<I> for Cli
where
    I: IO,
{
    fn run(&self, io: I) -> Result<RetCode, AnyError> {
        let storage = Storage::open(self.storage_path()?.to_str().ok_or("invalid path")?)?;

        match &self.cmd {
            Sub::Keys(sub) => sub.run((io, storage)),
        }
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
