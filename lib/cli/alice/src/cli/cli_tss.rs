use digest::Digest;
use ff::PrimeField;
use group::{Group, GroupEncoding};
use structopt::StructOpt;

use crate::AnyError;

use super::{Cli, CliRun};

mod cli_frost;

#[derive(Debug, StructOpt)]
pub struct CliTss<F, G, H> {
    #[structopt(subcommand)]
    cmd: Cmd<F, G, H>,
}

#[derive(Debug, StructOpt)]
enum Cmd<F, G, H> {
    Frost(cli_frost::CliFrost<F, G, H>),
}

impl<F, G, H> CliRun<&Cli<F, G, H>> for CliTss<F, G, H>
where
    F: PrimeField,
    G: Group<Scalar = F> + GroupEncoding,
    H: Digest,
{
    fn run(&self, cli: &Cli<F, G, H>) -> Result<(), AnyError> {
        match &self.cmd {
            Cmd::Frost(sub) => sub.run((self, cli)),
        }
    }
}
