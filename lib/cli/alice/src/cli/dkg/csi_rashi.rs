use std::collections::HashMap;
use std::io::Write;

use common_interop::curve_select::CurveSelect;
use common_interop::types::{Point, Scalar};
use ff::PrimeField;
use group::{Group, GroupEncoding};
use rand::RngCore;
use serde::{Deserialize, Serialize};
use specialize_call::specialize_call;
use structopt::StructOpt;

use cli_storage::{Storage, Table};

use crate::caps::IO;
use crate::data::{Key, S4Share};
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
    #[structopt(long, short, env = "ALICE_CURVE")]
    curve: CurveSelect,

    #[structopt(name = "KEY-ID")]
    key_id: String,
}

#[derive(Debug, StructOpt)]
struct CmdAggregate {
    #[structopt(name = "KEY-ID")]
    key_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Session {
    curve: CurveSelect,
    threshold: usize,
    s4_xs: Vec<Scalar>,
    this: usize,

    s4_y: Scalar,
    commitment: Vec<Point>,
}

pub fn run(
    csi_rashi: &CmdCsiRashi,
    rng: impl RngCore,
    io: impl IO,
    storage: Storage,
) -> Result<RetCode, AnyError> {
    match &csi_rashi.cmd {
            Cmd::Reset(sub) => run_reset(io, storage, sub),
            Cmd::Deal(sub) =>
                specialize_call!(run_deal, (rng, io, storage, sub), sub.curve, [
                    (CurveSelect::Secp256k1 => k256::Scalar, k256::ProjectivePoint),
                    (CurveSelect::Ed25519 => curve25519::scalar::Scalar, curve25519::edwards::EdwardsPoint),
                    (CurveSelect::Ristretto25519 => curve25519::scalar::Scalar, curve25519::ristretto::RistrettoPoint),
                ]).ok_or("Unsupported curve")?,
            Cmd::Aggregate(sub) =>
                run_aggregate(io, storage, sub),
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
    let key_id = deal.key_id.as_str();
    let curve = deal.curve;

    let tab_sessions = sessions_table(&storage)?;
    let tab_keys = keys_table(&storage)?;

    if tab_sessions.get(key_id)?.is_some() {
        writeln!(io.stderr(), "The session is already dealt: {}", key_id)?;
        return Ok(1)
    }
    if tab_keys.get(key_id)?.is_some() {
        writeln!(io.stderr(), "Key already exists: {}", key_id)?;
        return Ok(1)
    }

    #[derive(Debug, Deserialize)]
    struct Input {
        threshold: usize,
        this: usize,
        shamir_xs: Vec<Scalar>,
    }
    #[derive(Serialize)]
    struct Output {
        commitment: Vec<Point>,
        deals: HashMap<Scalar, Scalar>,
    }

    let input: Input = serde_yaml::from_reader(io.stdin())?;

    let threshold = input.threshold;

    let s4_xs = input
        .shamir_xs
        .iter()
        .map(|x| x.restore::<F>(curve))
        .collect::<Result<Vec<_>, _>>()?;

    let mut s4_ys = vec![F::ZERO; s4_xs.len()];

    let secret = F::random(&mut rng);
    let commitment = csi_rashi_dkg::deal::<F, G, MAX_THRESHOLD>(
        &mut rng,
        threshold,
        &secret,
        s4_xs.as_ref(),
        s4_ys.as_mut(),
    )?;

    let commitment = commitment
        .into_iter()
        .map(|p| Point::from_value(curve, p))
        .take(threshold)
        .collect::<Vec<_>>();
    let mut s4_xs = s4_xs.into_iter().map(|s| Scalar::from_value(curve, s)).collect::<Vec<_>>();
    let mut s4_ys = s4_ys.into_iter().map(|s| Scalar::from_value(curve, s)).collect::<Vec<_>>();

    let s4_y = s4_ys.remove(input.this);
    assert!(tab_sessions
        .insert(
            key_id,
            &Session {
                curve,
                threshold,
                s4_xs: s4_xs.clone(),
                this: input.this,
                s4_y,
                commitment: commitment.clone()
            }
        )?
        .is_none());
    let _s4_x = s4_xs.remove(input.this);

    let output = Output { commitment, deals: s4_xs.into_iter().zip(s4_ys).collect() };

    serde_yaml::to_writer(io.stdout(), &output)?;

    Ok(0)
}

fn run_aggregate(
    io: impl IO,
    storage: Storage,
    aggregate: &CmdAggregate,
) -> Result<RetCode, AnyError> {
    let key_id = aggregate.key_id.as_str();
    let tab_sessions = sessions_table(&storage)?;
    let Some(session) = tab_sessions.get(key_id)? else {
        writeln!(io.stderr(), "The session isn't dealt: {}", key_id)?;
        return Ok(1)
    };

    specialize_call!(run_aggregate_typed, (io, storage, aggregate, &session), session.curve, [
                    (CurveSelect::Secp256k1 => k256::Scalar, k256::ProjectivePoint),
                    (CurveSelect::Ed25519 => curve25519::scalar::Scalar, curve25519::edwards::EdwardsPoint),
                    (CurveSelect::Ristretto25519 => curve25519::scalar::Scalar, curve25519::ristretto::RistrettoPoint),
                ]).ok_or("Unsupported curve")?
}

fn run_aggregate_typed<F: PrimeField, G: Group<Scalar = F> + GroupEncoding>(
    io: impl IO,
    storage: Storage,
    aggregate: &CmdAggregate,
    session: &Session,
) -> Result<RetCode, AnyError> {
    let key_id = aggregate.key_id.as_str();
    let tab_sessions = sessions_table(&storage)?;
    let tab_keys = keys_table(&storage)?;

    let threshold = session.threshold;
    let curve = session.curve;

    if tab_keys.get(key_id)?.is_some() {
        writeln!(io.stderr(), "Key already exists: {}", key_id)?;
        return Ok(1)
    }

    #[derive(Debug, Deserialize)]
    struct Input {
        commitments: HashMap<Scalar, Vec<Point>>,
        deals: HashMap<Scalar, Scalar>,
    }

    let input: Input = serde_yaml::from_reader(io.stdin())?;

    let mut complaints = vec![false; session.s4_xs.len()];
    let mut vss_commitments = vec![];
    let mut shamir_ys = vec![];

    let own_commitment = session
        .commitment
        .iter()
        .map(|s| s.restore::<G>(curve))
        .collect::<Result<Vec<_>, _>>()?;
    let own_s4_y = session.s4_y.restore::<F>(curve)?;
    vss_commitments.push(own_commitment);
    shamir_ys.push(own_s4_y);

    for s4_x in session.s4_xs.iter() {
        if s4_x == &session.s4_xs[session.this] {
            continue
        }

        let vss_commitment = input
            .commitments
            .get(s4_x)
            .ok_or(format!("missing commitment (from {:?})", s4_x))?;
        let deal = input.deals.get(s4_x).ok_or(format!("missing deal (from {:?})", s4_x))?;

        vss_commitments.push(
            vss_commitment
                .iter()
                .map(|p| p.restore::<G>(curve))
                .collect::<Result<Vec<_>, _>>()?,
        );
        shamir_ys.push(deal.restore::<F>(curve)?);
    }

    let own_s4_x = session.s4_xs[session.this].restore::<F>(curve)?;

    let (s4_y, public_key) = csi_rashi_dkg::aggregate::<F, G>(
        vss_commitments.as_ref(),
        &own_s4_x,
        shamir_ys.as_ref(),
        &mut complaints,
    )?;

    let public_key = Point::from_value(curve, public_key);

    assert!(tab_keys
        .insert(
            key_id,
            &Key::S4Share(S4Share {
                curve,
                threshold,
                public_key,
                x: Scalar::from_value(curve, own_s4_x),
                y: Scalar::from_value(curve, s4_y),
            })
        )?
        .is_none());
    assert!(tab_sessions.remove(key_id)?.is_some());

    Ok(0)
}

fn sessions_table(storage: &Storage) -> Result<Table<Session>, AnyError> {
    Table::open(storage)
}

fn keys_table(storage: &Storage) -> Result<Table<Key>, AnyError> {
    Table::open(storage)
}
