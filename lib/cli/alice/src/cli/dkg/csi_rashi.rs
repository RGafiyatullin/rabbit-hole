use std::collections::HashMap;
use std::io::Write;

use common_interop::curve_select::CurveSelect;
use common_interop::types::{Point, Scalar};
use ff::PrimeField;
use group::{Group, GroupEncoding};
use rand::RngCore;
use serde::{Deserialize, Serialize};
use structopt::StructOpt;

use cli_storage::{Storage, Table};

use crate::caps::IO;
use crate::cli::{CliRun, CliRunnable};
use crate::data::key::Key;
use crate::{AnyError, RetCode};

const MAX_THRESHOLD: usize = 32;

#[derive(Debug, StructOpt)]
pub struct CmdCsiRashi {
    #[structopt(subcommand)]
    cmd: Cmd,
}

#[derive(Debug, StructOpt)]
enum Cmd {
    Reset(CmdReset),
    Deal(CmdDeal),
    Aggregate(CmdAggregate),
}

#[derive(Debug, StructOpt)]
struct CmdReset {
    #[structopt(name = "KEY-ID")]
    key_id: String,
}

#[derive(Debug, StructOpt)]
struct CmdDeal {
    #[structopt(long, short)]
    curve: CurveSelect,

    #[structopt(name = "KEY-ID")]
    key_id: String,
}

#[derive(Debug, StructOpt)]
struct CmdAggregate {
    #[structopt(long, short)]
    curve: CurveSelect,

    #[structopt(name = "KEY-ID")]
    key_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Session {
    s4_x: Scalar,
    s4_y: Scalar,
    commitment: Vec<Point>,
}

impl<I: IO, R: RngCore> CliRun<(R, I, Storage)> for CmdCsiRashi {
    fn run(&self, (rng, io, storage): (R, I, Storage)) -> Result<RetCode, AnyError> {
        match &self.cmd {
            Cmd::Reset(sub) => run_reset(io, storage, sub),
            Cmd::Deal(sub) =>
                specialize_call!(run_deal, (rng, io, storage, sub), sub.curve, [
                    (CurveSelect::Secp256k1 => k256::Scalar, k256::ProjectivePoint),
                    (CurveSelect::Ed25519 => curve25519::scalar::Scalar, curve25519::edwards::EdwardsPoint),
                    (CurveSelect::Ristretto25519 => curve25519::scalar::Scalar, curve25519::ristretto::RistrettoPoint),
                ]).ok_or("Unsupported curve")?,
            Cmd::Aggregate(sub) =>
                specialize_call!(run_aggregate, (io, storage, sub), sub.curve, [
                    (CurveSelect::Secp256k1 => k256::Scalar, k256::ProjectivePoint),
                    (CurveSelect::Ed25519 => curve25519::scalar::Scalar, curve25519::edwards::EdwardsPoint),
                    (CurveSelect::Ristretto25519 => curve25519::scalar::Scalar, curve25519::ristretto::RistrettoPoint),
                ]).ok_or("Unsupported curve")?
        }
    }
}

fn run_reset(io: impl IO, storage: Storage, reset: &CmdReset) -> Result<RetCode, AnyError> {
    let table = sessions_table(&storage)?;

    if table.remove(&reset.key_id)?.is_some() {
        writeln!(io.stderr(), "Session reset: {}", reset.key_id)?;
        Ok(0)
    } else {
        writeln!(io.stderr(), "No such session: {}", reset.key_id)?;
        Ok(1)
    }
}

fn run_deal<F: PrimeField, G: Group<Scalar = F> + GroupEncoding>(
    mut rng: impl RngCore,
    io: impl IO,
    storage: Storage,
    deal: &CmdDeal,
) -> Result<RetCode, AnyError> {
    let tab_sessions = sessions_table(&storage)?;
    let tab_keys = keys_table(&storage)?;

    if tab_sessions.get(&deal.key_id)?.is_some() {
        writeln!(io.stderr(), "The session is already dealt: {}", deal.key_id)?;
        return Ok(1)
    }
    if tab_keys.get(&deal.key_id)?.is_some() {
        writeln!(io.stderr(), "Key already exists: {}", deal.key_id)?;
        return Ok(1)
    }

    #[derive(Deserialize)]
    struct Input {
        threshold: usize,
        this: usize,
        shamir_xs: Vec<Scalar>,
    }
    #[derive(Serialize)]
    struct Output {
        threshold: usize,
        commitment: Vec<Point>,
        deals: HashMap<Scalar, Scalar>,
    }

    let input: Input = serde_yaml::from_reader(io.stdin())?;

    let threshold = input.threshold;

    let mut shamir_xs = input
        .shamir_xs
        .iter()
        .map(|x| x.restore::<F>(deal.curve))
        .collect::<Result<Vec<_>, _>>()?;
    let mut shamir_ys = vec![F::ZERO; shamir_xs.len()];

    let secret = F::random(&mut rng);
    let commitment = csi_rashi_dkg::deal::<F, G, MAX_THRESHOLD>(
        &mut rng,
        threshold,
        &secret,
        shamir_xs.as_ref(),
        shamir_ys.as_mut(),
    )?;

    let s4_x = Scalar::new(deal.curve, shamir_xs.remove(input.this));
    let s4_y = Scalar::new(deal.curve, shamir_ys.remove(input.this));
    let commitment = commitment.into_iter().map(|p| Point::new(deal.curve, p)).collect::<Vec<_>>();

    assert!(tab_sessions.insert(&deal.key_id, &Session { s4_x, s4_y, commitment: commitment.clone() })?.is_none());

    let deals = shamir_xs.into_iter().zip(shamir_ys).map(|(x, y)| 
        (
            Scalar::new(deal.curve, x),
            Scalar::new(deal.curve, y),
        )).collect();

    let output = Output {
        threshold,
        commitment,
        deals,
    };

    serde_yaml::to_writer(io.stdout(), &output)?;

    Ok(1)
}

fn run_aggregate<F: PrimeField, G: Group<Scalar = F> + GroupEncoding>(
    io: impl IO,
    storage: Storage,
    aggregate: &CmdAggregate,
) -> Result<RetCode, AnyError> {
    writeln!(
        io.stderr(),
        "aggregate({}) F: {}; G: {}",
        aggregate.key_id,
        std::any::type_name::<F>(),
        std::any::type_name::<G>()
    )?;
    Ok(1)
}

fn sessions_table(storage: &Storage) -> Result<Table<Session>, AnyError> {
    Table::open(storage)
}

fn keys_table(storage: &Storage) -> Result<Table<Key>, AnyError> {
    Table::open(storage)
}
