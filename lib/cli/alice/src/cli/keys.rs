use std::io::Write;

use structopt::StructOpt;

use cli_storage::{Storage, Table};

use crate::caps::IO;
use crate::cli::CliRun;
use crate::data::key::Key;
use crate::{AnyError, RetCode};

#[derive(Debug, StructOpt)]
pub struct CmdKeys {
    #[structopt(subcommand)]
    cmd: Sub,
}

#[derive(Debug, StructOpt)]
enum Sub {
    List(CmdKeysList),
    Rm(CmdKeyRm),
    Add(CmdKeyAdd),
    Get(CmdKeyGet),
}

#[derive(Debug, StructOpt)]
pub struct CmdKeyGet {
    #[structopt(name = "KEY-ID")]
    pub key_id: String,
}

#[derive(Debug, StructOpt)]
pub struct CmdKeyRm {
    #[structopt(name = "KEY-ID")]
    pub key_id: String,
}

#[derive(Debug, StructOpt)]
pub struct CmdKeyAdd {
    #[structopt(name = "KEY-ID")]
    pub key_id: String,
}

#[derive(Debug, StructOpt)]
pub struct CmdKeysList {
    #[structopt(name = "KEY-ID-PREFIX", default_value = "")]
    key_id_prefix: String,
}

impl<I: IO> CliRun<(I, Storage)> for CmdKeys {
    fn run(&self, (io, storage): (I, Storage)) -> Result<RetCode, crate::AnyError> {
        match &self.cmd {
            Sub::List(sub) => sub.run((io, storage)),
            Sub::Rm(sub) => sub.run((io, storage)),
            Sub::Add(sub) => sub.run((io, storage)),
            Sub::Get(sub) => sub.run((io, storage)),
        }
    }
}

impl<I: IO> CliRun<(I, Storage)> for CmdKeysList {
    fn run(&self, (io, storage): (I, Storage)) -> Result<RetCode, crate::AnyError> {
        let table = keys_table(&storage)?;

        let mut ids = vec![];
        for item in table.select(&self.key_id_prefix) {
            let (id, _key) = item?;
            ids.push(id);
        }

        serde_yaml::to_writer(io.stdout(), &ids)?;

        Ok(0)
    }
}

impl<I: IO> CliRun<(I, Storage)> for CmdKeyRm {
    fn run(&self, (io, storage): (I, Storage)) -> Result<RetCode, crate::AnyError> {
        let table = keys_table(&storage)?;

        if table.remove(&self.key_id)?.is_some() {
            writeln!(io.stderr(), "Key removed: {:?}", self.key_id)?;
            Ok(0)
        } else {
            writeln!(io.stderr(), "Key does not exist: {:?}", self.key_id)?;
            Ok(1)
        }
    }
}

impl<I: IO> CliRun<(I, Storage)> for CmdKeyGet {
    fn run(&self, (io, storage): (I, Storage)) -> Result<RetCode, crate::AnyError> {
        let table = keys_table(&storage)?;

        if let Some(key) = table.get(&self.key_id)? {
            serde_yaml::to_writer(io.stdout(), &key)?;
            Ok(0)
        } else {
            writeln!(io.stderr(), "Key does not exist: {:?}", self.key_id)?;
            Ok(1)
        }
    }
}

impl<I: IO> CliRun<(I, Storage)> for CmdKeyAdd {
    fn run(&self, (io, storage): (I, Storage)) -> Result<RetCode, crate::AnyError> {
        let table = keys_table(&storage)?;

        if !table.get(&self.key_id)?.is_some() {
            let key: Key = serde_yaml::from_reader(io.stdin())?;
            assert!(table.insert(&self.key_id, &key)?.is_none());
            Ok(0)
        } else {
            writeln!(io.stderr(), "The key already exists: {:?}", self.key_id)?;
            Ok(1)
        }
    }
}

fn keys_table(storage: &Storage) -> Result<Table<Key>, AnyError> {
    Table::open(storage)
}
