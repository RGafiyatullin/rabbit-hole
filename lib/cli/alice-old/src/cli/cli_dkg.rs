use ff::PrimeField;
use group::{Group, GroupEncoding};
use structopt::StructOpt;

use crate::AnyError;

use super::{Cli, CliRun};

mod cli_csi_rashi;

#[derive(Debug, StructOpt)]
pub struct CliDkg<F, G, H> {
    #[structopt(subcommand)]
    cmd: Cmd<F, G, H>,
}

#[derive(Debug, StructOpt)]
enum Cmd<F, G, H> {
    CsiRashi(cli_csi_rashi::CliSciRashi<F, G, H>),
}

impl<F, G, H> CliRun<&Cli<F, G, H>> for CliDkg<F, G, H>
where
    F: PrimeField,
    G: Group<Scalar = F> + GroupEncoding,
{
    fn run(&self, cli: &Cli<F, G, H>) -> Result<(), AnyError> {
        match &self.cmd {
            Cmd::CsiRashi(sub) => sub.run((self, cli)),
        }
    }
}
