use ff::PrimeField;
use group::{Group, GroupEncoding};
use rand::RngCore;
use serde::{Deserialize, Serialize};
use specialize_call::specialize_call;
use structopt::StructOpt;

use cli_storage::{Storage, Table};
use common_interop::curve_select::CurveSelect;
use common_interop::transcript::Transcript;
use common_interop::types::{Point, Scalar};

use crate::caps::IO;
use crate::data::{FullKey, Key};
use crate::{transcript, AnyError, RetCode};

#[derive(Debug, StructOpt)]
pub enum CmdSign {
    Schnorr(CmdSignSchnorr),
}

#[derive(Debug, StructOpt)]
pub struct CmdSignSchnorr {
    #[structopt(long, short)]
    key_id: String,
}

pub fn run(
    cmd: &CmdSign,
    rng: impl RngCore,
    io: impl IO,
    storage: Storage,
) -> Result<RetCode, AnyError> {
    match cmd {
        CmdSign::Schnorr(sub) => run_sign_schnorr(sub, rng, io, storage),
    }
}

pub fn run_sign_schnorr(
    cmd: &CmdSignSchnorr,
    rng: impl RngCore,
    io: impl IO,
    storage: Storage,
) -> Result<RetCode, AnyError> {
    let tab_keys = keys_table(&storage)?;
    let Key::FullKey(full_key) = tab_keys.get(&cmd.key_id)?.ok_or("No such key")? else { return Err("the key should be a Full-Key".into()) };
    let curve = full_key.curve;

    specialize_call!(
        run_sign_schnorr_typed, (&full_key, rng, io),
        curve,
        [
            (CurveSelect::Secp256k1 => k256::Scalar, k256::ProjectivePoint),
            (CurveSelect::Ed25519 => curve25519::scalar::Scalar, curve25519::edwards::EdwardsPoint),
            (CurveSelect::Ristretto25519 => curve25519::scalar::Scalar, curve25519::ristretto::RistrettoPoint),
        ]
    ).ok_or(format!("Unsupported curve: {}", curve))?
}

pub fn run_sign_schnorr_typed<F: PrimeField, G: Group<Scalar = F> + GroupEncoding>(
    full_key: &FullKey,
    rng: impl RngCore,
    io: impl IO,
) -> Result<RetCode, AnyError> {
    #[derive(Deserialize)]
    struct Input {
        transcript: Transcript,
        instance_key: Option<Scalar>,
    }

    #[derive(Serialize)]
    struct Output {
        y: Point,
        r: Point,
        s: Scalar,
    }

    let curve = full_key.curve;
    let g = G::generator();
    let x = full_key.value.restore::<F>(curve)?;
    let y = g * x;

    let input: Input = serde_yaml::from_reader(io.stdin())?;

    let k = input
        .instance_key
        .map(|s| s.restore::<F>(curve))
        .transpose()?
        .unwrap_or_else(|| F::random(rng));
    let c = transcript::produce_challenge(&input.transcript, &y, &(g * k))?;

    let (s, r) = schnorr_proof::prove(g, &x, &k, c);

    serde_yaml::to_writer(
        io.stdout(),
        &Output {
            y: Point::from_value(curve, y),
            r: Point::from_value(curve, r),
            s: Scalar::from_value(curve, s),
        },
    )?;

    Ok(0)
}

fn keys_table(storage: &Storage) -> Result<Table<Key>, AnyError> {
    Table::open(storage)
}
