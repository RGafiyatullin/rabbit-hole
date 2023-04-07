use std::marker::PhantomData;

use common_interop::transcript::Transcript;
use common_interop::types::{Point, Scalar};
use ff::PrimeField;
use group::{Group, GroupEncoding};
use serde::{Deserialize, Serialize};
use structopt::StructOpt;

use crate::cli::cli_tss::cli_frost::data::{Commitment, Nonce};
use crate::AnyError;

use super::{Cli, CliFrost, CliRun, CliTss};

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
{
    fn run(
        &self,
        (_frost, _tss, _cli): (&CliFrost<F, G, H>, &CliTss<F, G, H>, &Cli<F, G, H>),
    ) -> Result<(), AnyError> {
        let input: Input<F, G> = serde_yaml::from_reader(std::io::stdin().lock())?;

        eprintln!("{:#?}", input);

        unimplemented!()
    }
}
