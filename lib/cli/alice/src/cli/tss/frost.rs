use cli_storage::Table;
use common_interop::curve_select::CurveSelect;
use common_interop::hash_function_select::HashFunctionSelect;
use common_interop::transcript::Transcript;
use common_interop::types::{Point, Scalar};
use digest::Digest;
use ff::PrimeField;
use group::{Group, GroupEncoding};
use rand::RngCore;
use serde::{Deserialize, Serialize};
use structopt::StructOpt;

use cli_storage::Storage;

use crate::caps::IO;
use crate::data::key::{Key, S4Share};
use crate::{AnyError, RetCode};

#[derive(Debug, StructOpt)]
pub struct CmdFrost {
    #[structopt(subcommand)]
    cmd: Cmd,
}

#[derive(Debug, StructOpt)]
enum Cmd {
    Prepare(CmdPrepare),
    Sign(CmdSign),
    Aggregate(CmdAggregate),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Nonces {
    d: Scalar,
    e: Scalar,
}

#[derive(Debug, StructOpt)]
struct CmdPrepare {
    #[structopt(long, short)]
    key_id: String,

    #[structopt(long, short)]
    count: usize,
}

#[derive(Debug, StructOpt)]
struct CmdSign {
    #[structopt(long, short)]
    key_id: String,

    #[structopt(long, short)]
    hash_function: HashFunctionSelect,
}

#[derive(Debug, StructOpt)]
struct CmdAggregate {
    #[structopt(long, short)]
    curve: CurveSelect,
}

pub fn run(
    frost: &CmdFrost,
    rng: impl RngCore,
    io: impl IO,
    storage: Storage,
) -> Result<RetCode, AnyError> {
    match &frost.cmd {
        Cmd::Prepare(sub) => run_prepare(sub, rng, io, storage),
        Cmd::Sign(sub) => run_sign(sub, io, storage),
        Cmd::Aggregate(sub) => run_aggregate(sub, io),
    }
}

// fn run_typed<F: PrimeField, G: Group<Scalar = F> + GroupEncoding>(
//     frost: &CmdFrost,
//     s4_share: &S4Share,
//     rng: impl RngCore,
//     io: impl IO,
//     storage: Storage,
// ) -> Result<RetCode, AnyError> {
//     match &frost.cmd {
//         Cmd::Prepare(sub) => run_prepare(sub, rng, io, storage),
//         Cmd::Sign(sub) => specialize_call!(
//             run_sign, (frost, s4_share, io, storage),
//             ((), (), sub.hash_function),
//             [(_ => F)],
//             [(_ => G)],
//             [(HashFunctionSelect::Sha2_256 => sha2::Sha256), (HashFunctionSelect::Sha3_256 =>
// sha3::Sha3_256)]         ).ok_or("Unsupported hash-function")?,
//         Cmd::Aggregate => unimplemented!(),
//     }
// }

fn run_prepare(
    prepare: &CmdPrepare,
    rng: impl RngCore,
    io: impl IO,
    storage: Storage,
) -> Result<RetCode, AnyError> {
    let tab_keys = keys_table(&storage)?;
    let Key::S4Share(s4_share) = tab_keys.get(&prepare.key_id)?.ok_or("No such key")? else {
        return Err("the key should be an S4-share".into());
    };
    let curve = s4_share.curve;
    specialize_call!(run_prepare_typed, (prepare, &s4_share, rng, io, storage), curve, [
        (CurveSelect::Secp256k1 => k256::Scalar, k256::ProjectivePoint),
        (CurveSelect::Ed25519 => curve25519::scalar::Scalar, curve25519::edwards::EdwardsPoint),
        (CurveSelect::Ristretto25519 => curve25519::scalar::Scalar, curve25519::ristretto::RistrettoPoint),
    ]).ok_or(format!("Unsupported curve: {}", curve))?
}

fn run_prepare_typed<F: PrimeField, G: Group<Scalar = F> + GroupEncoding>(
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
        tab_nonces.insert(&nonce_key(&prepare.key_id, pd, pe), &Nonces { d, e })?;
    }

    serde_yaml::to_writer(io.stdout(), &commitments)?;

    Ok(0)
}

fn run_sign(sign: &CmdSign, io: impl IO, storage: Storage) -> Result<RetCode, AnyError> {
    let tab_keys = keys_table(&storage)?;
    let Key::S4Share(s4_share) = tab_keys.get(&sign.key_id)?.ok_or("No such key")? else {
        return Err("the key should be an S4-share".into());
    };
    let curve = s4_share.curve;
    let hash_function = sign.hash_function;

    specialize_call!(
        run_sign_typed,
        (sign, &s4_share, io, storage),
        (curve, hash_function),
        [
            (CurveSelect::Secp256k1 => k256::Scalar, k256::ProjectivePoint),
            (CurveSelect::Ed25519 => curve25519::scalar::Scalar, curve25519::edwards::EdwardsPoint),
            (CurveSelect::Ristretto25519 => curve25519::scalar::Scalar, curve25519::ristretto::RistrettoPoint),
        ],
        [
            (HashFunctionSelect::Sha2_256 => sha2::Sha256),
            (HashFunctionSelect::Sha3_256 => sha3::Sha3_256)
        ]
    ).ok_or(format!("Unsupported curve or hash-function: {}/{}", curve, hash_function))?
}

fn run_sign_typed<F: PrimeField, G: Group<Scalar = F> + GroupEncoding, H: Digest>(
    sign: &CmdSign,
    s4_share: &S4Share,
    io: impl IO,
    storage: Storage,
) -> Result<RetCode, AnyError> {
    let curve = s4_share.curve;

    let tab_nonces = nonces_table(&storage)?;

    #[derive(Deserialize)]
    struct Input {
        transcript: Transcript,
        signers: Vec<(Scalar, Point, Point)>,
    }
    #[derive(Serialize)]
    struct Output {
        y: Point,
        r: Point,
        z: Scalar,
    }

    let input: Input = serde_yaml::from_reader(io.stdin())?;

    if input.signers.len() != s4_share.threshold {
        return Err(format!(
            "Invalid threshold [expected: {}; commitments-count: {}]",
            s4_share.threshold,
            input.signers.len()
        )
        .into())
    }

    let participant_id = input.signers.iter().position(|(x, _, _)| x == &s4_share.x).ok_or(
        format!("Proposed commitments do not contain this key-share's `x`: {}", s4_share.x),
    )?;
    let public_key = s4_share.public_key.restore::<G>(curve)?;

    let (_, cd, ce) = &input.signers[participant_id];
    let nonce_key = nonce_key(&sign.key_id, cd, ce);

    let (shamir_xs, commitments): (Vec<_>, Vec<_>) = {
        let tmp = input
            .signers
            .iter()
            .map(|(x, cd, ce)| {
                let x = x.restore::<F>(curve)?;
                let cd = cd.restore::<G>(curve)?;
                let ce = ce.restore::<G>(curve)?;
                Ok((x, (cd, ce)))
            })
            .collect::<Result<Vec<_>, AnyError>>()?;
        tmp.into_iter().unzip()
    };
    let Some(nonces) = tab_nonces.remove(&nonce_key)? else {
        return Err(format!("Unknown commitment: {}-{}", cd, ce).into())
    };

    let shamir_y = s4_share.y.restore::<F>(curve)?;

    let (y, r, z) = frost_tss::sign::<F, G, H>(
        &public_key,
        participant_id,
        &shamir_y,
        &shamir_xs,
        &(nonces.d.restore::<F>(curve)?, nonces.e.restore::<F>(curve)?),
        &commitments,
        |y, r| transcript::produce_challenge(&input.transcript, y, r).expect("Invalid transcript"),
    );

    serde_yaml::to_writer(
        io.stdout(),
        &Output {
            y: Point::from_value(curve, y),
            r: Point::from_value(curve, r),
            z: Scalar::from_value(curve, z),
        },
    )?;

    Ok(0)
}

fn run_aggregate(aggregate: &CmdAggregate, io: impl IO) -> Result<RetCode, AnyError> {
    unimplemented!()
}

fn nonce_key(key_id: &str, cd: &Point, ce: &Point) -> String {
    format!("{}[{}-{}]", key_id, cd, ce)
}

fn keys_table(storage: &Storage) -> Result<Table<Key>, AnyError> {
    Table::open(storage)
}

fn nonces_table(storage: &Storage) -> Result<Table<Nonces>, AnyError> {
    Table::open(storage)
}

mod transcript {
    use cli_storage::AnyError;
    use common_interop::hash_function_select::HashFunctionSelect;
    use common_interop::transcript::{Input, KnownPoint, Transcript};
    use digest::Digest;
    use ff::PrimeField;
    use group::{Group, GroupEncoding};

    pub fn produce_challenge<F, G>(t: &Transcript, y: &G, r: &G) -> Result<F, AnyError>
    where
        F: PrimeField,
        G: Group<Scalar = F> + GroupEncoding,
    {
        match t.hash_function {
            HashFunctionSelect::Sha3_256 => produce_challenge_1::<F, G, sha3::Sha3_256>(t, y, r),
            HashFunctionSelect::Sha2_256 => produce_challenge_1::<F, G, sha2::Sha256>(t, y, r),
        }
    }

    fn produce_challenge_1<F, G, H>(t: &Transcript, y: &G, r: &G) -> Result<F, AnyError>
    where
        F: PrimeField,
        G: Group<Scalar = F> + GroupEncoding,
        H: Digest,
    {
        let mut hasher = H::new();

        for input in t.input.iter() {
            match input {
                Input::Hex(h) => hasher.update(hex::decode(h.as_str())?),
                Input::Text(t) => hasher.update(t),
                Input::Point(KnownPoint::Y) => hasher.update(y.to_bytes()),
                Input::Point(KnownPoint::R) => hasher.update(r.to_bytes()),
            }
        }

        Ok(utils::bytes_to_scalar(hasher.finalize().as_ref()))
    }
}
