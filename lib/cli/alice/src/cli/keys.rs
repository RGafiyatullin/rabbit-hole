use std::io::Write;

use structopt::StructOpt;

use cli_storage::{Storage, Table};

use crate::caps::IO;
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

pub fn run(keys: &CmdKeys, io: impl IO, storage: Storage) -> Result<RetCode, crate::AnyError> {
    match &keys.cmd {
        Sub::List(sub) => run_list(sub, io, storage),
        Sub::Rm(sub) => run_rm(sub, io, storage),
        Sub::Add(sub) => run_add(sub, io, storage),
        Sub::Get(sub) => run_get(sub, io, storage),
    }
}

fn run_list(list: &CmdKeysList, io: impl IO, storage: Storage) -> Result<RetCode, crate::AnyError> {
    let table = keys_table(&storage)?;

    let mut ids = vec![];
    for item in table.select(&list.key_id_prefix) {
        let (id, _key) = item?;
        ids.push(id);
    }

    serde_yaml::to_writer(io.stdout(), &ids)?;

    Ok(0)
}

fn run_rm(rm: &CmdKeyRm, io: impl IO, storage: Storage) -> Result<RetCode, crate::AnyError> {
    let table = keys_table(&storage)?;

    if table.remove(&rm.key_id)?.is_some() {
        writeln!(io.stderr(), "Key removed: {:?}", rm.key_id)?;
        Ok(0)
    } else {
        writeln!(io.stderr(), "Key does not exist: {:?}", rm.key_id)?;
        Ok(1)
    }
}

fn run_get(get: &CmdKeyGet, io: impl IO, storage: Storage) -> Result<RetCode, crate::AnyError> {
    let table = keys_table(&storage)?;

    if let Some(key) = table.get(&get.key_id)? {
        serde_yaml::to_writer(io.stdout(), &key)?;
        Ok(0)
    } else {
        writeln!(io.stderr(), "Key does not exist: {:?}", get.key_id)?;
        Ok(1)
    }
}

fn run_add(add: &CmdKeyAdd, io: impl IO, storage: Storage) -> Result<RetCode, crate::AnyError> {
    let table = keys_table(&storage)?;

    if !table.get(&add.key_id)?.is_some() {
        let key: Key = serde_yaml::from_reader(io.stdin())?;
        assert!(table.insert(&add.key_id, &key)?.is_none());
        Ok(0)
    } else {
        writeln!(io.stderr(), "The key already exists: {:?}", add.key_id)?;
        Ok(1)
    }
}

fn keys_table(storage: &Storage) -> Result<Table<Key>, AnyError> {
    Table::open(storage)
}
