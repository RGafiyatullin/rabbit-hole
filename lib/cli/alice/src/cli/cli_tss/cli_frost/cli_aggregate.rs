use std::marker::PhantomData;

use common_interop::transcript::Transcript;
use common_interop::types::{Point, Scalar};
use digest::Digest;
use ff::PrimeField;
use group::{Group, GroupEncoding};
use serde::{Deserialize, Serialize};
use serde_json::json;
use structopt::StructOpt;

use crate::cli::cli_tss::cli_frost::data::Commitment;
use crate::AnyError;

use super::{transcript, Cli, CliFrost, CliRun, CliTss};

#[derive(Debug, Clone, Deserialize)]
#[serde(bound(deserialize = "F: ::ff::PrimeField, G: ::group::GroupEncoding"))]
struct Input<F, G> {
    transcript: Transcript,
    shamir_xs: Vec<Scalar<F>>,
    commitments: Vec<Commitment<G>>,
    shards: Vec<Shard<F, G>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(bound(deserialize = "F: ::ff::PrimeField, G: ::group::GroupEncoding"))]
struct Shard<F, G> {
    y_i: Point<G>,
    r_i: Point<G>,
    z_i: Scalar<F>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(bound(serialize = "F: ::ff::PrimeField, G: ::group::GroupEncoding",))]
struct Output<F, G> {
    y: Point<G>,
    r: Point<G>,
    z: Scalar<F>,
}

#[derive(Debug, StructOpt)]
pub struct CliAggregate<F, G, H> {
    #[structopt(skip)]
    _pd: PhantomData<(F, G, H)>,
}

impl<F, G, H> CliRun<(&CliFrost<F, G, H>, &CliTss<F, G, H>, &Cli<F, G, H>)>
    for CliAggregate<F, G, H>
where
    F: PrimeField,
    G: Group<Scalar = F> + GroupEncoding,
    H: Digest,
{
    fn run(
        &self,
        (_frost, _tss, _cli): (&CliFrost<F, G, H>, &CliTss<F, G, H>, &Cli<F, G, H>),
    ) -> Result<(), AnyError> {
        let input: Input<F, G> = serde_yaml::from_reader(std::io::stdin().lock())?;

        let shamir_xs = input.shamir_xs.into_iter().map(Scalar::into_inner).collect::<Vec<_>>();
        let commitments = input
            .commitments
            .into_iter()
            .map(|c| (c.cd.into_inner(), c.ce.into_inner()))
            .collect::<Vec<_>>();

        let shards = input
            .shards
            .into_iter()
            .map(|s| (s.y_i.into_inner(), s.r_i.into_inner(), s.z_i.into_inner()))
            .collect::<Vec<_>>();

        let mut complaints = vec![false; shamir_xs.len()];
        let (y, r, z) = frost_tss::aggregate::<F, G, H>(
            &shards[..],
            &shamir_xs[..],
            &commitments[..],
            &mut complaints[..],
            |y, r| {
                transcript::produce_challenge(&input.transcript, y, r).expect("Invalid transcript")
            },
        )?;

        serde_yaml::to_writer(
            std::io::stdout().lock(),
            &json!({
                "signature": Output {
                    y: y.into(),
                    r: r.into(),
                    z: z.into(),
                },
            }),
        )?;

        Ok(())
    }
}
