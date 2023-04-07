use std::marker::PhantomData;

use common_interop::types::{Point, S4Share, Scalar};
use ff::PrimeField;
use group::{Group, GroupEncoding};
use serde_json::json;
use structopt::StructOpt;

use crate::AnyError;

use super::{Cli, CliDkg, CliRun, CliSciRashi};

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(bound(deserialize = "F: ::ff::PrimeField, G: ::group::GroupEncoding"))]
struct InboundDeal<F, G> {
    commitment: Vec<Point<G>>,
    shamir_y: Scalar<F>,
}

type Input<F, G> = Vec<InboundDeal<F, G>>;

#[derive(Debug, Clone, serde::Serialize)]
#[serde(bound(serialize = "F: ::ff::PrimeField, G: ::group::GroupEncoding"))]
struct Output<F, G> {
    public_key: Point<G>,
    shamir_x: Scalar<F>,
}

#[derive(Debug, StructOpt)]
pub struct CliAggregateDeals<F, G, H> {
    #[structopt(skip)]
    _pd: PhantomData<(F, G, H)>,
}

#[derive(Debug, StructOpt)]
enum Cmd {}

impl<'a, F, G, H> CliRun<(&'a CliSciRashi<F, G, H>, &'a CliDkg<F, G, H>, &'a Cli<F, G, H>)>
    for CliAggregateDeals<F, G, H>
where
    F: PrimeField,
    G: Group<Scalar = F> + GroupEncoding,
{
    fn run(
        &self,
        (csi_rashi, _dkg, cli): (&'a CliSciRashi<F, G, H>, &'a CliDkg<F, G, H>, &'a Cli<F, G, H>),
    ) -> Result<(), AnyError> {
        let key_id = &csi_rashi.key_id;

        let Some(session) = csi_rashi.sessions_table(cli)?
            .get(key_id)? else { return Err("".into()) };

        let input: Input<F, G> = serde_yaml::from_reader(std::io::stdin().lock())?;

        let shamir_x = session.shamir_x;

        let mut commitments =
            vec![session.commitment.into_iter().map(Point::into_inner).collect::<Vec<_>>()];
        let mut shamir_ys = vec![session.shamir_y.into_inner()];

        for inbound_deal in input {
            commitments.push(
                inbound_deal.commitment.into_iter().map(Point::into_inner).collect::<Vec<_>>(),
            );
            shamir_ys.push(inbound_deal.shamir_y.into_inner());
        }

        let mut complaints = vec![false; commitments.len()];

        let (shamir_y, public_key) = csi_rashi_dkg::aggregate::<F, G>(
            &commitments[..],
            shamir_x.as_ref(),
            &shamir_ys[..],
            &mut complaints[..],
        )?;

        let public_key = Point::from(public_key);
        let shamir_y = Scalar::from(shamir_y);

        let s4_share = S4Share { public_key, shamir_x, shamir_y };
        csi_rashi.s4_shares_table(cli)?.insert(key_id, &s4_share)?;
        csi_rashi.sessions_table(cli)?.remove(key_id)?;

        serde_yaml::to_writer(
            std::io::stdout().lock(),
            &json!({ "aggregate_deals": Output {
            public_key,
            shamir_x,
        } }),
        )?;

        Ok(())
    }
}
