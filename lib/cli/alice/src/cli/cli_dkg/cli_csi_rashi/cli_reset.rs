use std::marker::PhantomData;

use cli_storage::Table;
use ff::PrimeField;
use group::GroupEncoding;
use serde_json::json;
use structopt::StructOpt;

use crate::AnyError;

use super::data::Session;
use super::{Cli, CliDkg, CliRun, CliSciRashi};

#[derive(Debug, StructOpt)]
pub struct CliReset<F, G, H> {
    #[structopt(skip)]
    _pd: PhantomData<(F, G, H)>,
}

impl<'a, F, G, H> CliRun<(&'a CliSciRashi<F, G, H>, &'a CliDkg<F, G, H>, &'a Cli<F, G, H>)>
    for CliReset<F, G, H>
where
    F: PrimeField,
    G: GroupEncoding,
{
    fn run(
        &self,
        (csi_rashi, _dkg, cli): (&'a CliSciRashi<F, G, H>, &'a CliDkg<F, G, H>, &'a Cli<F, G, H>),
    ) -> Result<(), AnyError> {
        if let Some(session) = Table::<Session<F, G>>::open(cli.open_storage()?, cli.curve)?
            .remove(&csi_rashi.key_id)?
        {
            serde_yaml::to_writer(std::io::stdout().lock(), &json!({ "removed": session }))?;
        }
        Ok(())
    }
}
