use common_interop::curve_select::CurveSelect;
use common_interop::hash_function_select::HashFunctionSelect;
use common_interop::transcript::Transcript;
use common_interop::types::{Point, Scalar};
use digest::Digest;
use ff::PrimeField;
use group::{Curve, Group, GroupEncoding};
use k256::elliptic_curve::point::AffineCoordinates;
use serde::Deserialize;
use structopt::StructOpt;

use crate::caps::IO;
use crate::{transcript, AnyError, RetCode};

#[derive(Debug, StructOpt)]
pub enum CmdVerify {
    Schnorr(CmdVerifySchnorr),
    Ecdsa(CmdVerifyEcdsa),
}

#[derive(Debug, StructOpt)]
struct CmdVerifySchnorr {
    #[structopt(long, short)]
    curve: CurveSelect,
}

#[derive(Debug, StructOpt)]
struct CmdVerifyEcdsa {
    #[structopt(long, short)]
    curve: CurveSelect,
    #[structopt(long, short)]
    hash_function: HashFunctionSelect,
}

pub fn run(verify: &CmdVerify, io: impl IO) -> Result<RetCode, AnyError> {
    match verify {
        CmdVerify::Schnorr(sub) => run_verify_schnorr(sub, io),
        CmdVerify::Ecdsa(sub) => run_verify_ecdsa(sub, io),
    }
}

fn run_verify_schnorr(cmd: &CmdVerifySchnorr, io: impl IO) -> Result<RetCode, AnyError> {
    let curve = cmd.curve;

    specialize_call!(
        run_verify_schnorr_typed, (cmd, io),
        curve, [
            (CurveSelect::Secp256k1 => k256::Scalar, k256::ProjectivePoint),
            (CurveSelect::Ed25519 => curve25519::scalar::Scalar, curve25519::edwards::EdwardsPoint),
            (CurveSelect::Ristretto25519 => curve25519::scalar::Scalar, curve25519::ristretto::RistrettoPoint),
        ]).ok_or(format!("Unsupported curve: {}", curve))?
}

fn run_verify_ecdsa(cmd: &CmdVerifyEcdsa, io: impl IO) -> Result<RetCode, AnyError> {
    let curve = cmd.curve;
    let hash_function = cmd.hash_function;

    specialize_call!(
        run_verify_ecdsa_typed, (cmd, io),
        curve,
        [
            (CurveSelect::Secp256k1 => k256::Scalar, k256::ProjectivePoint),
        ]
    )
    .ok_or(format!("Unsupported curve: {}", curve))?
}

fn run_verify_ecdsa_typed<F, G>(cmd: &CmdVerifyEcdsa, io: impl IO) -> Result<RetCode, AnyError>
where
    F: PrimeField,
    G: Group<Scalar = F> + GroupEncoding + Curve,
    G::AffineRepr: AffineCoordinates<FieldRepr = F::Repr>,
{
    unimplemented!()
}

fn run_verify_schnorr_typed<F: PrimeField, G: Group<Scalar = F> + GroupEncoding>(
    cmd: &CmdVerifySchnorr,
    io: impl IO,
) -> Result<RetCode, AnyError> {
    #[derive(Debug, Deserialize)]
    struct Signature {
        r: Point,
        y: Point,
        s: Scalar,
    }
    #[derive(Debug, Deserialize)]
    struct Input {
        transcript: Transcript,
        signature: Signature,
    }

    let curve = cmd.curve;

    let input: Input = serde_yaml::from_reader(io.stdin())?;

    let r = input.signature.r.restore::<G>(curve)?;
    let y = input.signature.y.restore::<G>(curve)?;
    let s = input.signature.s.restore::<F>(curve)?;

    let c = transcript::produce_challenge(&input.transcript, &y, &r)?;

    serde_yaml::to_writer(io.stdout(), &schnorr_proof::verify(G::generator(), y, c, s, r))?;

    Ok(0)
}
