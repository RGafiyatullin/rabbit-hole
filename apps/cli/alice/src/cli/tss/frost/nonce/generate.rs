use ff::PrimeField;
use group::{Group, GroupEncoding};
use structopt::StructOpt;

use crate::curve::Curve;
use crate::AnyError;

use super::{Cli, Frost, Nonce, Tss};

#[derive(Debug, StructOpt)]
pub struct Generate {
    #[structopt(long, short)]
    count: usize,
}

impl Generate {
    pub async fn run(
        &self,
        nonce: &Nonce,
        frost: &Frost,
        tss: &Tss,
        cli: &Cli,
    ) -> Result<(), AnyError> {
        eprintln!("curve: {}", frost.curve);

        match frost.curve {
            Curve::Secp256k1 =>
                self.run_for_curve::<k256::Scalar, k256::ProjectivePoint>(nonce, frost, tss, cli)
                    .await,
            Curve::Ed25519 =>
                self.run_for_curve::<curve25519::scalar::Scalar, curve25519::edwards::EdwardsPoint>(
                    nonce, frost, tss, cli,
                )
                .await,
            Curve::Ristretto25519 => self
                .run_for_curve::<curve25519::scalar::Scalar, curve25519::ristretto::RistrettoPoint>(
                    nonce, frost, tss, cli,
                )
                .await,
        }
    }

    async fn run_for_curve<F, G>(
        &self,
        _nonce: &Nonce,
        frost: &Frost,
        _tss: &Tss,
        cli: &Cli,
    ) -> Result<(), AnyError>
    where
        F: PrimeField,
        G: Group<Scalar = F> + GroupEncoding,
    {
        let mut rng = cli.rng();
        let mut nonces = vec![(F::ZERO, F::ZERO); self.count];
        let mut commitments = vec![(G::identity(), G::identity()); self.count];

        eprintln!("generating {} nonce-pairs...", self.count);
        frost_tss::preprocess::<F, G>(&mut rng, &mut nonces[..], &mut commitments[..]);
        eprintln!("done!");

        cli.with_secrets_manager(|mut sm| {
            let commitments = &commitments;
            async move {
                for ((d, e), (cd, ce)) in nonces.into_iter().zip(commitments.iter().copied()) {
                    let key = format!(
                        "{}/{}:{}",
                        cli.secrets_ns.tss_frost_nonce(frost.curve),
                        hex::encode(cd.to_bytes()),
                        hex::encode(ce.to_bytes()),
                    );

                    let value =
                        format!("{}:{}", hex::encode(d.to_repr()), hex::encode(e.to_repr()),);

                    sm.set(key.as_str(), value);
                }
                sm.save()
            }
        })
        .await??;

        for (cd, ce) in commitments {
            println!("{}:{}", hex::encode(cd.to_bytes()), hex::encode(ce.to_bytes()),);
        }

        Ok(())
    }
}
