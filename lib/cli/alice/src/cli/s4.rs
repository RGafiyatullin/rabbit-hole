use std::io::Write;

use ff::PrimeField;
use group::{Group, GroupEncoding};
use rand::RngCore;
use specialize_call::specialize_call;
use structopt::StructOpt;

use cli_storage::{Storage, Table};
use common_interop::curve_select::CurveSelect;
use common_interop::types::{Point, Scalar};

use crate::caps::IO;
use crate::data::{Key, S4Share, S4};
use crate::{AnyError, RetCode};

#[derive(Debug, StructOpt)]
pub enum CmdS4 {
    Export(CmdS4Export),
    Gen(CmdS4Gen),
    Import(CmdS4Import),
    IssueShare(CmdS4IssueShare),
    List(CmdS4List),
    Rm(CmdS4Rm),
}

#[derive(Debug, StructOpt)]
pub struct CmdS4Gen {
    #[structopt(long, short)]
    key_id: String,

    #[structopt(long, short)]
    threshold: usize,

    #[structopt(name = "SCHEME-ID")]
    s4_id: String,
}

#[derive(Debug, StructOpt)]
pub struct CmdS4IssueShare {
    #[structopt(long, short)]
    key_id: String,

    #[structopt(name = "SCHEME-ID")]
    s4_id: String,

    #[structopt(name = "SHAMIR-X")]
    shamir_x: Scalar,
}

#[derive(Debug, StructOpt)]
pub struct CmdS4List {
    #[structopt(name = "SCHEME-ID-PREFIX", default_value = "")]
    prefix: String,
}

#[derive(Debug, StructOpt)]
pub struct CmdS4Import {
    #[structopt(name = "SCHEME-ID")]
    s4_id: String,
}

#[derive(Debug, StructOpt)]
pub struct CmdS4Export {
    #[structopt(name = "SCHEME-ID")]
    s4_id: String,
}

#[derive(Debug, StructOpt)]
pub struct CmdS4Rm {
    #[structopt(long, short)]
    s4_id: String,
}

pub fn run(
    cmd: &CmdS4,
    rng: impl RngCore,
    io: impl IO,
    storage: Storage,
) -> Result<RetCode, AnyError> {
    match cmd {
        CmdS4::Export(sub) => run_export(sub, io, storage),
        CmdS4::Gen(sub) => run_gen(sub, rng, storage),
        CmdS4::Import(sub) => run_import(sub, io, storage),
        CmdS4::IssueShare(sub) => run_issue_share(sub, storage),
        CmdS4::List(sub) => run_list(sub, io, storage),
        CmdS4::Rm(sub) => run_rm(sub, io, storage),
    }
}

fn run_issue_share(cmd: &CmdS4IssueShare, storage: Storage) -> Result<RetCode, AnyError> {
    let tab_s4 = s4_table(&storage)?;
    let tab_keys = keys_table(&storage)?;

    if tab_keys.get(&cmd.key_id)?.is_some() {
        return Err(format!("Key already exists: {}", cmd.key_id).into())
    }
    let Some(s4) = tab_s4.get(&cmd.s4_id)? else { return Err(format!("No such scheme: {}", cmd.key_id).into()) };

    let curve = s4.curve;

    fn calculate_shamir_y<F: PrimeField, G: Group<Scalar = F> + GroupEncoding>(
        curve: CurveSelect,
        polynomial: &[Scalar],
        shamir_x: &Scalar,
    ) -> Result<(Scalar, Point), AnyError> {
        let polynomial = polynomial
            .iter()
            .map(|s| s.restore::<F>(curve))
            .collect::<Result<Vec<_>, AnyError>>()?;
        let shamir_x = shamir_x.restore::<F>(curve)?;

        let mut acc = F::ZERO;
        let mut x_to_n = F::ONE;

        let public_key = G::generator() * polynomial[0];
        for c in polynomial {
            acc += x_to_n * c;
            x_to_n *= shamir_x;
        }

        Ok((Scalar::from_value(curve, acc), Point::from_value(curve, public_key)))
    }

    let threshold = s4.polynomial.len().saturating_sub(1);
    let (shamir_y, public_key) = specialize_call!(calculate_shamir_y, (curve, s4.polynomial.as_ref(), &cmd.shamir_x), curve, [
        (CurveSelect::Secp256k1 => k256::Scalar, k256::ProjectivePoint),
        (CurveSelect::Ed25519 => curve25519::scalar::Scalar, curve25519::edwards::EdwardsPoint),
        (CurveSelect::Ristretto25519 => curve25519::scalar::Scalar, curve25519::ristretto::RistrettoPoint)
    ]).ok_or(format!("Unsupported curve: {}", curve))??;

    let key_share = S4Share { curve, threshold, public_key, x: cmd.shamir_x.clone(), y: shamir_y };
    tab_keys.insert(&cmd.key_id, &Key::S4Share(key_share))?;

    Ok(0)
}

fn run_gen(cmd: &CmdS4Gen, mut rng: impl RngCore, storage: Storage) -> Result<RetCode, AnyError> {
    let tab_s4 = s4_table(&storage)?;
    let tab_keys = keys_table(&storage)?;

    let Key::FullKey(full_key) = tab_keys
        .get(&cmd.key_id)?
        .ok_or(format!("No such key: {}", cmd.key_id))? 
        else { return Err("Should be a full-key".into()) };

    let curve = full_key.curve;

    if tab_s4.get(&cmd.s4_id)?.is_some() {
        return Err(format!("Scheme already exists: {}", cmd.s4_id).into())
    }

    let polynomial = std::iter::once(Some(full_key.value))
        .chain((0..cmd.threshold).map(|_| {
            specialize_call!(random_scalar, (curve, &mut rng), curve, [
            (CurveSelect::Secp256k1 => k256::Scalar),
            (CurveSelect::Ed25519 | CurveSelect::Ristretto25519 => curve25519::scalar::Scalar),
        ])
        }))
        .collect::<Option<Vec<_>>>()
        .ok_or(format!("Unsupported curve: {}", curve))?;

    let s4 = S4 { curve, polynomial };

    tab_s4.insert(&cmd.s4_id, &s4)?;

    Ok(0)
}

fn run_import(cmd: &CmdS4Import, io: impl IO, storage: Storage) -> Result<RetCode, AnyError> {
    let table = s4_table(&storage)?;

    if table.get(&cmd.s4_id)?.is_none() {
        let s4: S4 = serde_yaml::from_reader(io.stdin())?;
        assert!(table.insert(&cmd.s4_id, &s4)?.is_none());
        Ok(0)
    } else {
        writeln!(io.stderr(), "The scheme already exists: {:?}", cmd.s4_id)?;
        Ok(1)
    }
}

fn run_export(cmd: &CmdS4Export, io: impl IO, storage: Storage) -> Result<RetCode, AnyError> {
    let table = s4_table(&storage)?;

    if let Some(key) = table.get(&cmd.s4_id)? {
        serde_yaml::to_writer(io.stdout(), &key)?;
        Ok(0)
    } else {
        writeln!(io.stderr(), "Scheme does not exist: {:?}", cmd.s4_id)?;
        Ok(1)
    }
}

fn run_list(cmd: &CmdS4List, io: impl IO, storage: Storage) -> Result<RetCode, AnyError> {
    let tab_s4 = s4_table(&storage)?;

    let mut ids = vec![];
    for item in tab_s4.select(&cmd.prefix) {
        let (id, _) = item?;
        ids.push(id);
    }

    serde_yaml::to_writer(io.stdout(), &ids)?;

    Ok(0)
}

fn run_rm(cmd: &CmdS4Rm, io: impl IO, storage: Storage) -> Result<RetCode, AnyError> {
    let table = s4_table(&storage)?;

    if table.remove(&cmd.s4_id)?.is_some() {
        writeln!(io.stderr(), "Scheme removed: {:?}", cmd.s4_id)?;
        Ok(0)
    } else {
        writeln!(io.stderr(), "Scheme does not exist: {:?}", cmd.s4_id)?;
        Ok(1)
    }
}

fn s4_table(storage: &Storage) -> Result<Table<S4>, AnyError> {
    Table::open(storage)
}

fn keys_table(storage: &Storage) -> Result<Table<Key>, AnyError> {
    Table::open(storage)
}

fn random_scalar<F: PrimeField>(curve: CurveSelect, rng: impl RngCore) -> Scalar {
    Scalar::from_value(curve, F::random(rng))
}
