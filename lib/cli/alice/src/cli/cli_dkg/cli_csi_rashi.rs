use cli_storage::Table;
use common_interop::types::S4Share;
use ff::PrimeField;
use group::{Group, GroupEncoding};
use structopt::StructOpt;

use crate::AnyError;

use self::data::Session;

use super::{Cli, CliDkg, CliRun};

mod cli_aggregate_deals;
mod cli_produce_deals;
mod cli_reset;

mod data;

#[derive(Debug, StructOpt)]
pub struct CliSciRashi<F, G, H> {
    #[structopt(long, short)]
    key_id: String,

    #[structopt(subcommand)]
    cmd: Cmd<F, G, H>,
}

#[derive(Debug, StructOpt)]
enum Cmd<F, G, H> {
    ProduceDeals(cli_produce_deals::CliProduceDeals<F, G, H>),
    AggregateDeals(cli_aggregate_deals::CliAggregateDeals<F, G, H>),
    Reset(cli_reset::CliReset<F, G, H>),
}

impl<'a, F, G, H> CliRun<(&'a CliDkg<F, G, H>, &'a Cli<F, G, H>)> for CliSciRashi<F, G, H>
where
    F: PrimeField,
    G: Group<Scalar = F> + GroupEncoding,
{
    fn run(&self, (dkg, cli): (&'a CliDkg<F, G, H>, &'a Cli<F, G, H>)) -> Result<(), AnyError> {
        match &self.cmd {
            Cmd::ProduceDeals(sub) => sub.run((self, dkg, cli)),
            Cmd::AggregateDeals(sub) => sub.run((self, dkg, cli)),
            Cmd::Reset(sub) => sub.run((self, dkg, cli)),
        }
    }
}

impl<F, G, H> CliSciRashi<F, G, H> {
    fn sessions_table(&self, cli: &Cli<F, G, H>) -> Result<Table<Session<F, G>>, AnyError> {
        let table = Table::<Session<F, G>>::open(cli.open_storage()?, cli.curve)?;
        Ok(table)
    }
    fn s4_shares_table(&self, cli: &Cli<F, G, H>) -> Result<Table<S4Share<F, G>>, AnyError> {
        let table = Table::<S4Share<F, G>>::open(cli.open_storage()?, cli.curve)?;
        Ok(table)
    }
}
