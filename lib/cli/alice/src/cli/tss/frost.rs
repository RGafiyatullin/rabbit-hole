use cli_storage::Table;
use common_interop::curve_select::CurveSelect;
use common_interop::types::{Point, Scalar};
use ff::PrimeField;
use group::{Group, GroupEncoding};
use rand::RngCore;
use serde::{Serialize, Deserialize};
use structopt::StructOpt;

use cli_storage::Storage;

use crate::caps::IO;
use crate::data::key::{Key, S4Share};
use crate::{AnyError, RetCode};

#[derive(Debug, StructOpt)]
pub struct CmdFrost {
    #[structopt(long, short)]
    key_id: String,

    #[structopt(subcommand)]
    cmd: Cmd,
}

#[derive(Debug, StructOpt)]
enum Cmd {
    Prepare(CmdPrepare),
    Sign,
    Aggregate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Nonces {
    d: Scalar,
    e: Scalar,
}

#[derive(Debug, StructOpt)]
struct CmdPrepare {
    #[structopt(long, short)]
    count: usize,
}

pub fn run(
    frost: &CmdFrost,
    rng: impl RngCore,
    io: impl IO,
    storage: Storage,
) -> Result<RetCode, AnyError> {
    let tab_keys = keys_table(&storage)?;
    let Key::S4Share(s4_share) = tab_keys.get(&frost.key_id)?.ok_or("No such key")? else {
        return Err("the key should be an S4-share".into());
    };

    let curve = s4_share.curve;

    specialize_call!(run_typed, (frost, &s4_share, rng, io, storage), curve, [
        (CurveSelect::Secp256k1 => k256::Scalar, k256::ProjectivePoint),
        (CurveSelect::Ed25519 => curve25519::scalar::Scalar, curve25519::edwards::EdwardsPoint),
        (CurveSelect::Ristretto25519 => curve25519::scalar::Scalar, curve25519::ristretto::RistrettoPoint),
    ]).ok_or("Unsupported curve")?
}

fn run_typed<F: PrimeField, G: Group<Scalar = F> + GroupEncoding>(
    frost: &CmdFrost,
    s4_share: &S4Share,
    rng: impl RngCore,
    io: impl IO,
    storage: Storage,
) -> Result<RetCode, AnyError> {
    match &frost.cmd {
        Cmd::Prepare(sub) => run_prepare::<F, G>(frost, sub, s4_share, rng, io, storage),
        Cmd::Sign => unimplemented!(),
        Cmd::Aggregate => unimplemented!(),
    }
}

fn run_prepare<F: PrimeField, G: Group<Scalar = F> + GroupEncoding>(
    frost: &CmdFrost,
    prepare: &CmdPrepare,
    s4_share: &S4Share,
    rng: impl RngCore,
    io: impl IO,
    storage: Storage,
) -> Result<RetCode, AnyError> {
    let curve = s4_share.curve;

    let tab_nonces = nonces_table(&storage)?;

    let mut nonces = vec![(F::ZERO, F::ZERO); prepare.count];
    let mut commitments = vec![(G::identity(), G::identity()); prepare.count];

    frost_tss::preprocess(rng, nonces.as_mut(), commitments.as_mut());

    let nonces = nonces
        .into_iter()
        .map(|(d, e)| ((Scalar::from_value(curve, d), Scalar::from_value(curve, e))))
        .collect::<Vec<_>>();
    let commitments = commitments
        .into_iter()
        .map(|(pd, pe)| (Point::from_value(curve, pd), Point::from_value(curve, pe)))
        .collect::<Vec<_>>();

    for i in 0..prepare.count {
        let (d, e) = nonces[i].clone();
        let (pd, pe) = &commitments[i];
        tab_nonces.insert(&nonce_key(&frost.key_id, pd, pe), &Nonces {
            d, e
        })?;
    }

    serde_yaml::to_writer(io.stdout(), &commitments)?;
    
    Ok(0)
}

fn nonce_key(key_id: &str, pd: &Point, pe: &Point) -> String {
    format!("{}[{}-{}]", key_id, pd, pe)
}


fn keys_table(storage: &Storage) -> Result<Table<Key>, AnyError> {
    Table::open(storage)
}

fn nonces_table(storage: &Storage) -> Result<Table<Nonces>, AnyError> {
    Table::open(storage)
}
