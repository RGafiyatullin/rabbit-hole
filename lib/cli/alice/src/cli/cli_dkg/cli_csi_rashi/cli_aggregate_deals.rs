use std::marker::PhantomData;

use cli_storage::Table;
use ff::PrimeField;
use group::{Group, GroupEncoding};
use structopt::StructOpt;

use crate::cli::cli_dkg::cli_csi_rashi::data::Session;
use crate::AnyError;

use super::{Cli, CliDkg, CliRun, CliSciRashi};

#[derive(Debug, StructOpt)]
pub struct CliAggregateDeals<F, G, H> {
    #[structopt(skip)]
    _pd: PhantomData<(F, G, H)>,
}

#[derive(Debug, StructOpt)]
enum Cmd {}

impl<'a, F, G, H> CliRun<(&'a CliSciRashi<F, G, H>, &'a CliDkg<F, G, H>, &'a Cli<F, G, H>)>
    for CliAggregateDeals<F, G, H>
where
    F: PrimeField,
    G: Group<Scalar = F> + GroupEncoding,
{
    fn run(
        &self,
        (_csi_rashi, _dkg, _cli): (&'a CliSciRashi<F, G, H>, &'a CliDkg<F, G, H>, &'a Cli<F, G, H>),
    ) -> Result<(), AnyError> {
        unimplemented!()
    }
}
