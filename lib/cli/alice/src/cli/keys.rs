use std::io::Write;

use common_interop::curve_select::CurveSelect;
use common_interop::types::Scalar;
use ff::PrimeField;
use rand::RngCore;
use specialize_call::specialize_call;
use structopt::StructOpt;

use cli_storage::{Storage, Table};

use crate::caps::IO;
use crate::data::{FullKey, Key};
use crate::{AnyError, RetCode};

#[derive(Debug, StructOpt)]
pub enum CmdKeys {
    Export(CmdKeyExport),
    Gen(CmdKeyGen),
    Import(CmdKeyImport),
    List(CmdKeysList),
    Rm(CmdKeyRm),
}

#[derive(Debug, StructOpt)]
pub struct CmdKeyGen {
    #[structopt(long, short, env = "ALICE_CURVE")]
    curve: CurveSelect,

    #[structopt(name = "KEY-ID")]
    key_id: String,
}

#[derive(Debug, StructOpt)]
pub struct CmdKeyExport {
    #[structopt(name = "KEY-ID")]
    key_id: String,
}

#[derive(Debug, StructOpt)]
pub struct CmdKeyRm {
    #[structopt(name = "KEY-ID")]
    key_id: String,
}

#[derive(Debug, StructOpt)]
pub struct CmdKeyImport {
    #[structopt(name = "KEY-ID")]
    key_id: String,
}

#[derive(Debug, StructOpt)]
pub struct CmdKeysList {
    #[structopt(name = "KEY-ID-PREFIX", default_value = "")]
    prefix: String,
}

pub fn run(
    keys: &CmdKeys,
    rng: impl RngCore,
    io: impl IO,
    storage: Storage,
) -> Result<RetCode, crate::AnyError> {
    match keys {
        CmdKeys::Gen(sub) => run_gen(sub, rng, io, storage),
        CmdKeys::List(sub) => run_list(sub, io, storage),
        CmdKeys::Rm(sub) => run_rm(sub, io, storage),
        CmdKeys::Import(sub) => run_import(sub, io, storage),
        CmdKeys::Export(sub) => run_export(sub, io, storage),
    }
}

fn run_list(list: &CmdKeysList, io: impl IO, storage: Storage) -> Result<RetCode, crate::AnyError> {
    let table = keys_table(&storage)?;

    let mut ids = vec![];
    for item in table.select(&list.prefix) {
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

fn run_export(
    export: &CmdKeyExport,
    io: impl IO,
    storage: Storage,
) -> Result<RetCode, crate::AnyError> {
    let table = keys_table(&storage)?;

    if let Some(key) = table.get(&export.key_id)? {
        serde_yaml::to_writer(io.stdout(), &key)?;
        Ok(0)
    } else {
        writeln!(io.stderr(), "Key does not exist: {:?}", export.key_id)?;
        Ok(1)
    }
}

fn run_gen(
    gen: &CmdKeyGen,
    rng: impl RngCore,
    io: impl IO,
    storage: Storage,
) -> Result<RetCode, crate::AnyError> {
    let curve = gen.curve;
    let table = keys_table(&storage)?;

    let value = specialize_call!(random_scalar, (curve, rng), curve, [
        (CurveSelect::Secp256k1 => k256::Scalar),
        (CurveSelect::Ed25519 | CurveSelect::Ristretto25519 => curve25519::scalar::Scalar),
    ])
    .ok_or(format!("Unsupported curve: {}", curve))?;
    if table.get(&gen.key_id)?.is_none() {
        let key = Key::FullKey(FullKey { curve, value });
        assert!(table.insert(&gen.key_id, &key)?.is_none());
        Ok(0)
    } else {
        writeln!(io.stderr(), "The key already exists: {:?}", gen.key_id)?;
        Ok(1)
    }
}

fn run_import(
    import: &CmdKeyImport,
    io: impl IO,
    storage: Storage,
) -> Result<RetCode, crate::AnyError> {
    let table = keys_table(&storage)?;

    if table.get(&import.key_id)?.is_none() {
        let key: Key = serde_yaml::from_reader(io.stdin())?;
        assert!(table.insert(&import.key_id, &key)?.is_none());
        Ok(0)
    } else {
        writeln!(io.stderr(), "The key already exists: {:?}", import.key_id)?;
        Ok(1)
    }
}

fn keys_table(storage: &Storage) -> Result<Table<Key>, AnyError> {
    Table::open(storage)
}

fn random_scalar<F: PrimeField>(curve: CurveSelect, rng: impl RngCore) -> Scalar {
    Scalar::from_value(curve, F::random(rng))
}
