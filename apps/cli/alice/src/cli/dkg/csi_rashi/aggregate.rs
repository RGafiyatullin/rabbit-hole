use std::io::BufRead;

use ff::PrimeField;
use group::{Group, GroupEncoding};
use structopt::StructOpt;

use crate::cli::dkg::csi_rashi::KeyShareShard;
use crate::common::Curve;
use crate::AnyError;

use super::{Cli, CsiRashi, Dkg};

#[derive(Debug, StructOpt)]
pub struct Aggregate {}

impl Aggregate {
    pub async fn run(&self, csi_rashi: &CsiRashi, dkg: &Dkg, cli: &Cli) -> Result<(), AnyError> {
        match csi_rashi.curve {
            Curve::Secp256k1 =>
                self.run_for_curve::<k256::Scalar, k256::ProjectivePoint>(csi_rashi, dkg, cli)
                    .await,
            Curve::Ed25519 =>
                self.run_for_curve::<curve25519::scalar::Scalar, curve25519::edwards::EdwardsPoint>(
                    csi_rashi, dkg, cli,
                )
                .await,
            Curve::Ristretto25519 => self
                .run_for_curve::<curve25519::scalar::Scalar, curve25519::ristretto::RistrettoPoint>(
                    csi_rashi, dkg, cli,
                )
                .await,
        }
    }

    async fn run_for_curve<F, G>(
        &self,
        csi_rashi: &CsiRashi,
        _dkg: &Dkg,
        cli: &Cli,
    ) -> Result<(), AnyError>
    where
        F: PrimeField,
        G: Group<Scalar = F> + GroupEncoding,
    {
        let key_id = csi_rashi.key_id.as_str();

        let own_share_storage_key =
            format!("{}/{}", cli.secrets_ns.dkg_csi_rashi_own_share(csi_rashi.curve), key_id);
        let s4_storage_key =
            format!("{}/{}", cli.secrets_ns.key_share_s4_for_curve(csi_rashi.curve), key_id);

        let shards = std::io::stdin()
            .lock()
            .lines()
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .map(|line| line.parse())
            .collect::<Result<Vec<KeyShareShard<F, G>>, _>>()?;

        let shards_count = shards.len();

        let (mut shamir_ys, mut commitments): (Vec<_>, Vec<_>) =
            shards.into_iter().map(|shard| (shard.shamir_y, shard.commitment)).unzip();
        let mut complaints = vec![false; shards_count];

        let (public_key, shamir_x) = cli
            .with_secrets_manager(|mut sm| async move {
                let own_share: KeyShareShard<F, G> = sm.get(&own_share_storage_key)?.parse()?;
                shamir_ys.push(own_share.shamir_y);
                commitments.push(own_share.commitment);
                complaints.push(false);

                let (key_share, public_key) = csi_rashi_dkg::aggregate(
                    &commitments[..],
                    &own_share.shamir_x,
                    &shamir_ys[..],
                    &mut complaints[..],
                )?;

                sm.set(
                    &s4_storage_key,
                    format!(
                        "{}:{}:{}",
                        hex::encode(public_key.to_bytes().as_ref()),
                        hex::encode(own_share.shamir_x.to_repr().as_ref()),
                        hex::encode(key_share.to_repr().as_ref())
                    ),
                );
                sm.remove(&own_share_storage_key)?;

                sm.save()?;

                Ok::<_, AnyError>((public_key, own_share.shamir_x))
            })
            .await??;

        println!(
            "{}:{}",
            hex::encode(public_key.to_bytes().as_ref()),
            hex::encode(shamir_x.to_repr().as_ref()),
        );

        Ok(())
    }
}
