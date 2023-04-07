use std::io::BufRead;

use digest::Digest;
use ff::PrimeField;
use group::{Group, GroupEncoding};
use structopt::StructOpt;

use crate::common::{decode_point, Curve, HashFunction};
use crate::AnyError;

use super::{Cli, Frost, Tss};

#[derive(Debug, StructOpt)]
pub struct Aggregate {
    #[structopt(long, short)]
    hash_function: HashFunction,
}

impl Aggregate {
    pub async fn run(&self, frost: &Frost, tss: &Tss, cli: &Cli) -> Result<(), AnyError> {
        match frost.curve {
            Curve::Secp256k1 =>
                self.run_for_curve::<k256::Scalar, k256::ProjectivePoint>(frost, tss, cli).await,
            Curve::Ed25519 =>
                self.run_for_curve::<curve25519::scalar::Scalar, curve25519::edwards::EdwardsPoint>(
                    frost, tss, cli,
                )
                .await,
            Curve::Ristretto25519 => self
                .run_for_curve::<curve25519::scalar::Scalar, curve25519::ristretto::RistrettoPoint>(
                    frost, tss, cli,
                )
                .await,
        }
    }

    async fn run_for_curve<F, G>(&self, frost: &Frost, tss: &Tss, cli: &Cli) -> Result<(), AnyError>
    where
        F: PrimeField,
        G: Group<Scalar = F> + GroupEncoding,
    {
        match self.hash_function {
            HashFunction::Sha3_256 =>
                self.run_for_curve_and_hash::<F, G, sha3::Sha3_256>(frost, tss, cli).await,
        }
    }

    async fn run_for_curve_and_hash<F, G, H>(
        &self,
        _frost: &Frost,
        _tss: &Tss,
        _cli: &Cli,
    ) -> Result<(), AnyError>
    where
        F: PrimeField,
        G: Group<Scalar = F> + GroupEncoding,
        H: Digest,
    {
        let mut stdin_lines = std::io::stdin().lock().lines();

        let (shamir_xs, commitments): (Vec<_>, Vec<_>) =
            // std::io::stdin().lock().lines().collect::<Result<Vec<_>, _>>()?
            (&mut stdin_lines).take_while(|l| l.as_ref().map(|l| !l.is_empty()).unwrap_or_default())
                .collect::<Result<Vec<_>, _>>()?
                .into_iter().map(|line| {
                    let (shamir_x, commitment) = line.split_once(':').ok_or("missing `:`")?;
                    let shamir_x: F = utils::bytes_to_scalar(hex::decode(shamir_x)?.as_ref());
                    let (cd, ce) = commitment.split_once(':').ok_or("missing `:`")?;

                    let cd:G = decode_point(cd).ok_or("bad point")?;
                    let ce:G = decode_point(ce).ok_or("bad point")?;

                    Ok(( shamir_x, (cd,ce) ))
                })
                .collect::<Result<Vec<_>, AnyError>>()?
                .into_iter()
                .unzip();

        let mut signature_shards = vec![];

        for line in (&mut stdin_lines)
            .take_while(|l| l.as_ref().map(|l| !l.is_empty()).unwrap_or_default())
            .collect::<Result<Vec<_>, _>>()?
        {
            let (_participant_id, line) = line.split_once(':').ok_or("missing `:`")?;
            let (y_i, line) = line.split_once(':').ok_or("missing `:`")?;
            let (r_i, z_i) = line.split_once(':').ok_or("missing `:`")?;

            let y_i: G = decode_point(y_i).ok_or("bad point")?;
            let r_i: G = decode_point(r_i).ok_or("bad point")?;
            let z_i: F = utils::bytes_to_scalar(hex::decode(z_i)?.as_ref());

            signature_shards.push((y_i, r_i, z_i));
        }

        let message = hex::decode(stdin_lines.collect::<Result<Vec<_>, _>>()?.join("\n").as_str())?;

        let mut complaints = vec![false; shamir_xs.len()];

        let (y, r, z) = frost_tss::aggregate::<F, G, H>(
            &signature_shards[..],
            &shamir_xs[..],
            &commitments[..],
            &mut complaints[..],
            |y, g| {
                utils::bytes_to_scalar(
                    H::new()
                        .chain_update(y.to_bytes())
                        .chain_update(g.to_bytes())
                        .chain_update(&message)
                        .finalize()
                        .as_ref(),
                )
            },
        )?;

        println!(
            "{}:{}:{}",
            hex::encode(y.to_bytes().as_ref()),
            hex::encode(r.to_bytes().as_ref()),
            hex::encode(z.to_repr().as_ref()),
        );

        Ok(())
    }
}
