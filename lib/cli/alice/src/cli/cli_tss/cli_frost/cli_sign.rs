use std::marker::PhantomData;

use common_interop::transcript::Transcript;
use common_interop::types::{Point, Scalar};
use digest::Digest;
use ff::PrimeField;
use group::{Group, GroupEncoding};
use serde::{Deserialize, Serialize};
use serde_json::json;
use structopt::StructOpt;

use crate::cli::cli_tss::cli_frost::data::{Commitment, Nonce};
use crate::AnyError;

use super::{transcript, Cli, CliFrost, CliRun, CliTss};

#[derive(Debug, Clone, Deserialize)]
#[serde(bound(deserialize = "F: ::ff::PrimeField, G: ::group::GroupEncoding"))]
struct Input<F, G> {
    transcript: Transcript,
    shamir_xs: Vec<Scalar<F>>,
    commitments: Vec<Commitment<G>>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(bound(serialize = "F: ::ff::PrimeField, G: ::group::GroupEncoding",))]
struct Output<F, G> {
    y_i: Point<G>,
    r_i: Point<G>,
    z_i: Scalar<F>,
}

#[derive(Debug, StructOpt)]
pub struct CliSign<F, G, H> {
    #[structopt(skip)]
    _pd: PhantomData<(F, G, H)>,
}

impl<F, G, H> CliRun<(&CliFrost<F, G, H>, &CliTss<F, G, H>, &Cli<F, G, H>)> for CliSign<F, G, H>
where
    F: PrimeField,
    G: Group<Scalar = F> + GroupEncoding,
    H: Digest,
{
    fn run(
        &self,
        (frost, _tss, cli): (&CliFrost<F, G, H>, &CliTss<F, G, H>, &Cli<F, G, H>),
    ) -> Result<(), AnyError> {
        let s4_key_id = &frost.key_id;
        let s4_shares_table = frost.s4_shares_table(cli)?;
        let nonces_table = frost.nonces_table(cli)?;

        let s4_share = s4_shares_table
            .get(s4_key_id)?
            .ok_or(format!("no such s4-share: {}", s4_key_id))?;

        let input: Input<F, G> = serde_yaml::from_reader(std::io::stdin().lock())?;

        let participant_id = input
            .shamir_xs
            .iter()
            .position(|x| *x == s4_share.shamir_x)
            .ok_or("could not resolve participant id")?;
        let commitment = input.commitments[participant_id];

        let shamir_xs = input.shamir_xs.into_iter().map(Scalar::into_inner).collect::<Vec<_>>();
        let commitments = input
            .commitments
            .into_iter()
            .map(|c| (c.cd.into_inner(), c.ce.into_inner()))
            .collect::<Vec<_>>();

        let nonce_storage_key = commitment.to_storage_key(s4_key_id);
        let nonce = nonces_table.remove(&nonce_storage_key)?.ok_or("nonce-pair not found")?;

        let nonce = (nonce.d.into_inner(), nonce.e.into_inner());

        let (y_i, r_i, z_i) = ::frost_tss::sign::<F, G, H>(
            s4_share.public_key.as_ref(),
            participant_id,
            s4_share.shamir_y.as_ref(),
            &shamir_xs[..],
            &nonce,
            &commitments[..],
            |y, r| {
                transcript::produce_challenge(&input.transcript, y, r).expect("Invalid transcript")
            },
        );

        serde_yaml::to_writer(
            std::io::stdout().lock(),
            &json!({
                "sign": Output {
                    y_i: y_i.into(),
                    r_i: r_i.into(),
                    z_i: z_i.into(),
                },
            }),
        )?;

        Ok(())
    }
}
