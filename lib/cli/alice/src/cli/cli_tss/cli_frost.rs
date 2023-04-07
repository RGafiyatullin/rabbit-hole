use cli_storage::Table;
use common_interop::s4_share::S4Share;
use digest::Digest;
use ff::PrimeField;
use group::{Group, GroupEncoding};
use structopt::StructOpt;

use crate::AnyError;

use super::{Cli, CliRun, CliTss};

mod cli_aggregate;
mod cli_nonce;
mod cli_sign;

mod data;
use data::Nonce;

mod transcript;

#[derive(Debug, StructOpt)]
pub struct CliFrost<F, G, H> {
    #[structopt(long, short)]
    key_id: String,

    #[structopt(subcommand)]
    cmd: Cmd<F, G, H>,
}

#[derive(Debug, StructOpt)]
enum Cmd<F, G, H> {
    Nonce(cli_nonce::CliNonce<F, G, H>),
    Sign(cli_sign::CliSign<F, G, H>),
    Aggregate(cli_aggregate::CliAggregate<F, G, H>),
}

impl<F, G, H> CliRun<(&CliTss<F, G, H>, &Cli<F, G, H>)> for CliFrost<F, G, H>
where
    F: PrimeField,
    G: Group<Scalar = F> + GroupEncoding,
    H: Digest,
{
    fn run(&self, (tss, cli): (&CliTss<F, G, H>, &Cli<F, G, H>)) -> Result<(), AnyError> {
        match &self.cmd {
            Cmd::Nonce(sub) => sub.run((self, tss, cli)),
            Cmd::Sign(sub) => sub.run((self, tss, cli)),
            Cmd::Aggregate(sub) => sub.run((self, tss, cli)),
        }
    }
}

impl<F, G, H> CliFrost<F, G, H> {
    fn s4_shares_table(&self, cli: &Cli<F, G, H>) -> Result<Table<S4Share<F, G>>, AnyError> {
        let table = Table::open(cli.open_storage()?, cli.curve)?;
        Ok(table)
    }

    fn nonces_table(&self, cli: &Cli<F, G, H>) -> Result<Table<Nonce<F>>, AnyError> {
        let table = Table::open(cli.open_storage()?, cli.curve)?;
        Ok(table)
    }
}
