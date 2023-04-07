use std::marker::PhantomData;

use ff::PrimeField;
use group::GroupEncoding;
use serde_json::json;
use structopt::StructOpt;

use crate::AnyError;

use super::{Cli, CliDkg, CliRun, CliSciRashi};

#[derive(Debug, StructOpt)]
pub struct CliReset<F, G, H> {
    #[structopt(skip)]
    _pd: PhantomData<(F, G, H)>,
}

impl<F, G, H> CliRun<(&CliSciRashi<F, G, H>, &CliDkg<F, G, H>, &Cli<F, G, H>)> for CliReset<F, G, H>
where
    F: PrimeField,
    G: GroupEncoding,
{
    fn run(
        &self,
        (csi_rashi, _dkg, cli): (&CliSciRashi<F, G, H>, &CliDkg<F, G, H>, &Cli<F, G, H>),
    ) -> Result<(), AnyError> {
        if let Some(session) = csi_rashi.sessions_table(cli)?.remove(&csi_rashi.key_id)? {
            serde_yaml::to_writer(std::io::stdout().lock(), &json!({ "removed": session }))?;
        }
        Ok(())
    }
}
