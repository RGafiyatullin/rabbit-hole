use std::io::BufRead;

use digest::Digest;
use ff::PrimeField;
use group::{Group, GroupEncoding};
use structopt::StructOpt;

use crate::common::{decode_point, Curve, HashFunction};
use crate::AnyError;

use super::{Cli, Frost, Tss};

#[derive(Debug, StructOpt)]
pub struct Sign {
    #[structopt(long, short)]
    key_id: String,

    #[structopt(long, short)]
    hash_function: HashFunction,
}

impl Sign {
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
        frost: &Frost,
        _tss: &Tss,
        cli: &Cli,
    ) -> Result<(), AnyError>
    where
        F: PrimeField,
        G: Group<Scalar = F> + GroupEncoding,
        H: Digest,
    {
        let secret_share_storage_key =
            format!("{}/{}", cli.secrets_ns.key_share_s4_for_curve(frost.curve), self.key_id);

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

        let message = hex::decode(stdin_lines.collect::<Result<Vec<_>, _>>()?.join("\n").as_str())?;

        let (public_key, shamir_x, shamir_y) = cli
            .with_secrets_manager(|sm| async move {
                let s = sm.get(secret_share_storage_key.as_str())?;
                let (public_key, s) = s.split_once(':').ok_or("missing `:`")?;
                let (shamir_x, shamir_y) = s.split_once(':').ok_or("missing `:`")?;

                let public_key: G = decode_point(public_key).ok_or("bad point")?;
                let shamir_x: F = utils::bytes_to_scalar(hex::decode(shamir_x)?.as_ref());
                let shamir_y: F = utils::bytes_to_scalar(hex::decode(shamir_y)?.as_ref());

                Ok::<_, AnyError>((public_key, shamir_x, shamir_y))
            })
            .await??;

        let participant_id = shamir_xs
            .iter()
            .position(|&x| x == shamir_x)
            .ok_or("couldn't determine own participant-id")?;
        let (cd, ce) = commitments[participant_id];

        let nonce_ready_storage_key = format!(
            "{}/{}:{}",
            cli.secrets_ns.tss_frost_nonce_ready(frost.curve),
            hex::encode(cd.to_bytes().as_ref()),
            hex::encode(ce.to_bytes().as_ref())
        );

        let (c, d) = cli
            .with_secrets_manager(|sm| async move {
                let s = sm.get(&nonce_ready_storage_key)?;
                let (d, e) = s.split_once(':').ok_or("missing `:`")?;

                let d: F = utils::bytes_to_scalar(hex::decode(d)?.as_ref());
                let e: F = utils::bytes_to_scalar(hex::decode(e)?.as_ref());

                Ok::<_, AnyError>((d, e))
            })
            .await??;

        let (y_i, r_i, z_i) = frost_tss::sign::<F, G, H>(
            &public_key,
            participant_id,
            &shamir_y,
            &shamir_xs[..],
            &(c, d),
            &commitments[..],
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
        );

        let nonce_ready_storage_key = format!(
            "{}/{}:{}",
            cli.secrets_ns.tss_frost_nonce_ready(frost.curve),
            hex::encode(cd.to_bytes().as_ref()),
            hex::encode(ce.to_bytes().as_ref())
        );
        let nonce_used_storage_key = format!(
            "{}/{}:{}",
            cli.secrets_ns.tss_frost_nonce_used(frost.curve),
            hex::encode(cd.to_bytes().as_ref()),
            hex::encode(ce.to_bytes().as_ref())
        );
        cli.with_secrets_manager(|mut sm| async move {
            sm.remove(&nonce_ready_storage_key)?;
            sm.set(&nonce_used_storage_key, String::new());
            sm.save()
        })
        .await??;

        println!(
            "{}:{}:{}:{}",
            participant_id,
            hex::encode(y_i.to_bytes().as_ref()),
            hex::encode(r_i.to_bytes().as_ref()),
            hex::encode(z_i.to_repr().as_ref()),
        );

        Ok(())
    }
}
