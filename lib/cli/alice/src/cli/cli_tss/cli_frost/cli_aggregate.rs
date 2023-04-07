use std::marker::PhantomData;

use ff::PrimeField;
use group::{Group, GroupEncoding};
use structopt::StructOpt;

use crate::cli::cli_tss::cli_frost::data::{Commitment, Nonce};
use crate::AnyError;

use super::{Cli, CliFrost, CliRun, CliTss};

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
        unimplemented!()
    }
}
