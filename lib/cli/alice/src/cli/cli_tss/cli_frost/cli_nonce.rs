use std::marker::PhantomData;

use ff::PrimeField;
use group::{Group, GroupEncoding};
use structopt::StructOpt;

use crate::cli::cli_tss::cli_frost::data::{Commitment, Nonce};
use crate::AnyError;

use super::{Cli, CliFrost, CliRun, CliTss};

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
            Cmd::Generate { count } => generate(self, frost, tss, cli, *count),
        }
    }
}

fn generate<F, G, H>(
    _nonce: &CliNonce<F, G, H>,
    frost: &CliFrost<F, G, H>,
    _tss: &CliTss<F, G, H>,
    cli: &Cli<F, G, H>,
    count: usize,
) -> Result<(), AnyError>
where
    F: PrimeField,
    G: Group<Scalar = F> + GroupEncoding,
{
    let rng = cli.rng();
    let s4_key_id = &frost.key_id;
    let s4_shares_table = frost.s4_shares_table(cli)?;
    let nonces_table = frost.nonces_table(cli)?;

    let _s4_share = s4_shares_table
        .get(s4_key_id)?
        .ok_or(format!("no such s4-share: {:?}", s4_key_id))?;

    let mut nonces = vec![(F::ZERO, F::ZERO); count];
    let mut commitments = vec![(G::identity(), G::identity()); count];

    frost_tss::preprocess::<F, G>(rng, &mut nonces[..], &mut commitments[..]);

    let mut output = vec![];

    for ((d, e), (cd, ce)) in nonces.into_iter().zip(commitments) {
        let nonce = Nonce { d: d.into(), e: e.into() };
        let commitment = Commitment { cd: cd.into(), ce: ce.into() };

        nonces_table.insert(commitment.to_storage_key(s4_key_id).as_str(), &nonce)?;
        output.push(commitment);
    }

    serde_yaml::to_writer(std::io::stdout().lock(), &output)?;

    Ok(())
}
