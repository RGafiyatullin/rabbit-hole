use std::marker::PhantomData;

use cli_storage::Table;
use common_interop::s4_share::S4Share;
use ff::PrimeField;
use group::GroupEncoding;
use serde_json::json;
use structopt::StructOpt;

use crate::AnyError;

use super::{Cli, CliRun};

#[derive(Debug, StructOpt)]
pub struct CliKeys<F, G, H> {
    #[structopt(subcommand)]
    cmd: Cmd,

    #[structopt(skip)]
    _pd: PhantomData<(F, G, H)>,
}

#[derive(Debug, StructOpt)]
enum Cmd {
    List,
}

impl<F, G, H> CliRun<&Cli<F, G, H>> for CliKeys<F, G, H>
where
    F: PrimeField,
    G: GroupEncoding,
{
    fn run(&self, cli: &Cli<F, G, H>) -> Result<(), AnyError> {
        match &self.cmd {
            Cmd::List => list(self, cli),
        }
    }
}

impl<F, G, H> CliKeys<F, G, H> {
    fn s4_shares_table(&self, cli: &Cli<F, G, H>) -> Result<Table<S4Share<F, G>>, AnyError> {
        let table = Table::open(cli.open_storage()?, cli.curve)?;
        Ok(table)
    }
}

fn list<F, G, H>(keys: &CliKeys<F, G, H>, cli: &Cli<F, G, H>) -> Result<(), AnyError>
where
    F: PrimeField,
    G: GroupEncoding,
{
    let s4_shares_table = keys.s4_shares_table(cli)?;

    let output = s4_shares_table.select("").try_fold(vec![], |mut output, kv_result| {
        kv_result.map(move |(key_id, key_share)| {
            output.push(json!({
                "key_id": key_id,
                "shamir_x": key_share.shamir_x,
            }));
            output
        })
    })?;

    serde_yaml::to_writer(std::io::stdout(), &json!({ "key_shares": output }))?;

    Ok(())
}
