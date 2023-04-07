use std::marker::PhantomData;

use ff::PrimeField;
use group::{Group, GroupEncoding};
use structopt::StructOpt;

use crate::AnyError;

use super::{Cli, CliFrost, CliRun, CliTss};

// mod cli_aggregate_deals;
// mod cli_produce_deals;
// mod cli_reset;

// mod data;

#[derive(Debug, StructOpt)]
pub struct CliNonce<F, G, H> {
    #[structopt(subcommand)]
    cmd: Cmd, /* <F, G, H> */

    #[structopt(skip)]
    _pd: PhantomData<(F, G, H)>,
}

#[derive(Debug, StructOpt)]
enum Cmd /* <F, G, H> */ {
    Generate {
        #[structopt(long, short)]
        count: usize,
    },
}

impl<F, G, H> CliRun<(&CliFrost<F, G, H>, &CliTss<F, G, H>, &Cli<F, G, H>)> for CliNonce<F, G, H>
where
    F: PrimeField,
    G: Group<Scalar = F> + GroupEncoding,
{
    fn run(
        &self,
        (frost, tss, cli): (&CliFrost<F, G, H>, &CliTss<F, G, H>, &Cli<F, G, H>),
    ) -> Result<(), AnyError> {
        match &self.cmd {
            Cmd::Generate { count } => generate(self, frost, tss, cli),
        }
    }
}

fn generate<F, G, H>(
    _nonce: &CliNonce<F, G, H>,
    _frost: &CliFrost<F, G, H>,
    _tss: &CliTss<F, G, H>,
    _cli: &Cli<F, G, H>,
) -> Result<(), AnyError> {
    unimplemented!()
}
