use ff::PrimeField;
use group::{Group, GroupEncoding};
use structopt::StructOpt;

use crate::curve::Curve;
use crate::AnyError;

use super::{Cli, Commitment, CsiRashi, Dkg, KeyShareShard, MAX_THRESHOLD};

#[derive(Debug, StructOpt)]
pub struct Deal {
    #[structopt(long, short)]
    threshold: usize,

    #[structopt(long, short = "x")]
    own_shamir_x: String,
}

impl Deal {
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
        let mut rng = cli.rng();

        let key_id = csi_rashi.key_id.as_str();

        let own_share_storage_key =
            format!("{}/{}", cli.secrets_ns.dkg_csi_rashi_own_share(csi_rashi.curve), key_id);

        let threshold = self.threshold;

        let own_shamir_x: F =
            utils::bytes_to_scalar(hex::decode(self.own_shamir_x.as_str())?.as_ref());
        let mut shamir_xs = std::io::stdin()
            .lines()
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .map(|s| {
                Ok::<_, AnyError>(utils::bytes_to_scalar::<F>(hex::decode(s.as_str())?.as_ref()))
            })
            .collect::<Result<Vec<F>, _>>()?;
        shamir_xs.push(own_shamir_x);

        let key_shards =
            cli.with_secrets_manager(|mut sm| async move {
                let Err(_e) = sm.get(&own_share_storage_key) else { return Err(format!("invalid state for {:?}", key_id).into()) };

                let secret = F::random(&mut rng);
                let mut shamir_ys = vec![F::ZERO; shamir_xs.len()];
                let commitment = csi_rashi_dkg::deal::<F, G, MAX_THRESHOLD>(
                    &mut rng,
                    threshold,
                    &secret,
                    &shamir_xs,
                    &mut shamir_ys[..]
                )?
                    .into_iter()
                    .take_while(|c| c.is_identity().unwrap_u8() == 0)
                    .collect();
                let commitment = Commitment(commitment);

                let own_shamir_x = shamir_xs.pop().expect("we have at least pushed our own shamir-x");
                let own_shamir_y = shamir_ys.pop().expect("we have at least pushed our own shamir-x");
                let own_share = KeyShareShard {
                    shamir_x: own_shamir_x,
                    shamir_y: own_shamir_y,
                    commitment: commitment.clone(),
                };

                sm.set(&own_share_storage_key, own_share.to_string());
                sm.save()?;

                let key_shards = shamir_xs.into_iter().zip(shamir_ys).map(move |(shamir_x, shamir_y)| KeyShareShard { shamir_x, shamir_y, commitment: commitment.clone() });

                Ok::<_, AnyError>(key_shards)
            }).await??;

        for key_shard in key_shards {
            println!("{}", key_shard)
        }

        Ok(())
    }
}
