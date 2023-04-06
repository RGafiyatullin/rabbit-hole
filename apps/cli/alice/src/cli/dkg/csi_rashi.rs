use std::sync::Arc;

use ff::PrimeField;
use group::GroupEncoding;
use structopt::StructOpt;

use crate::curve::Curve;
use crate::AnyError;

use super::{Cli, Dkg};

mod aggregate;
mod cancel;
mod deal;

const MAX_THRESHOLD: usize = 10;

#[derive(Debug, StructOpt)]
pub struct CsiRashi {
    #[structopt(long, short)]
    curve: Curve,

    #[structopt(long, short = "k")]
    key_id: String,

    #[structopt(subcommand)]
    cmd: Cmd,
}

#[derive(Debug, StructOpt)]
enum Cmd {
    Deal(deal::Deal),
    Aggregate(aggregate::Aggregate),
    Cancel(cancel::Cancel),
}

impl CsiRashi {
    pub async fn run(&self, dkg: &Dkg, cli: &Cli) -> Result<(), AnyError> {
        match &self.cmd {
            Cmd::Deal(sub) => sub.run(self, dkg, cli).await,
            Cmd::Aggregate(sub) => sub.run(self, dkg, cli).await,
            Cmd::Cancel(sub) => sub.run(self, dkg, cli).await,
        }
    }
}

#[derive(Debug, Clone)]
pub struct KeyShareShard<F, G> {
    shamir_x: F,
    shamir_y: F,
    commitment: Commitment<G>,
}

impl<F, G> std::fmt::Display for KeyShareShard<F, G>
where
    F: PrimeField,
    G: GroupEncoding,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{}:{}",
            hex::encode(self.shamir_x.to_repr().as_ref()),
            hex::encode(self.shamir_y.to_repr().as_ref()),
            self.commitment
        )
    }
}

#[derive(Debug, Clone)]
pub struct Commitment<G>(Arc<[G]>);

impl<G> AsRef<[G]> for Commitment<G> {
    fn as_ref(&self) -> &[G] {
        self.0.as_ref()
    }
}

impl<F, G> std::str::FromStr for KeyShareShard<F, G>
where
    F: PrimeField,
    G: GroupEncoding,
{
    type Err = AnyError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((shamir_x, s)) = s.split_once(':') else { return Err("':' missing".into()) };
        let Some((shamir_y, commitment)) = s.split_once(':') else { return Err("':' missing".into()) };

        let shamir_x = utils::bytes_to_scalar(hex::decode(shamir_x)?.as_ref());
        let shamir_y = utils::bytes_to_scalar(hex::decode(shamir_y)?.as_ref());
        let commitment = commitment.parse()?;

        Ok(Self { shamir_x, shamir_y, commitment })
    }
}

impl<G> std::fmt::Display for Commitment<G>
where
    G: GroupEncoding,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let commitment =
            self.0.iter().map(|c| hex::encode(c.to_bytes().as_ref())).collect::<Vec<_>>();
        write!(f, "{}", commitment.join(" "))
    }
}

impl<G> std::str::FromStr for Commitment<G>
where
    G: GroupEncoding,
{
    type Err = AnyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let commitment = s
            .split_ascii_whitespace()
            .map(|x| {
                let mut repr: G::Repr = Default::default();
                hex::decode_to_slice(x, repr.as_mut())?;
                let c = G::from_bytes(&repr);
                if c.is_some().unwrap_u8() == 1 {
                    Ok(c.unwrap())
                } else {
                    Err("decode error".into())
                }
            })
            .collect::<Result<_, AnyError>>()?;

        Ok(Self(commitment))
    }
}
