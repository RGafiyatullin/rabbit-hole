use cli_storage::Table;
use common_interop::types::S4Share;
use ff::PrimeField;
use group::{Group, GroupEncoding};
use structopt::StructOpt;

use crate::AnyError;

use super::{Cli, CliRun, CliTss};

mod cli_nonce;

// mod data;

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
}

impl<F, G, H> CliRun<(&CliTss<F, G, H>, &Cli<F, G, H>)> for CliFrost<F, G, H>
where
    F: PrimeField,
    G: Group<Scalar = F> + GroupEncoding,
{
    fn run(&self, (tss, cli): (&CliTss<F, G, H>, &Cli<F, G, H>)) -> Result<(), AnyError> {
        match &self.cmd {
            Cmd::Nonce(sub) => sub.run((self, tss, cli)),
        }
    }
}

impl<F, G, H> CliFrost<F, G, H> {
    fn s4_shares_table(&self, cli: &Cli<F, G, H>) -> Result<Table<S4Share<F, G>>, AnyError> {
        let table = Table::<S4Share<F, G>>::open(cli.open_storage()?, cli.curve)?;
        Ok(table)
    }
}
