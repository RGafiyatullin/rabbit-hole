use std::collections::HashMap;
use std::marker::PhantomData;

use cli_storage::Table;
use common_interop::types::{Point, Scalar};
use ff::PrimeField;
use group::{Group, GroupEncoding};
use serde_json::json;
use structopt::StructOpt;

use crate::AnyError;

use super::data::Session;
use super::{Cli, CliDkg, CliRun, CliSciRashi};

const MAX_THRESHOLD: usize = 64;

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(bound(deserialize = "F: ::ff::PrimeField"))]
pub struct ProduceDealsInput<F> {
    threshold: usize,
    own_shamir_x: Scalar<F>,
    shamir_xs: Vec<Scalar<F>>,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(bound(serialize = "F: ::ff::PrimeField, G: ::group::GroupEncoding"))]
pub struct ProduceDealsOutput<F, G> {
    threshold: usize,
    commitment: Vec<Point<G>>,
    secret_deals: HashMap<Scalar<F>, Scalar<F>>,
}

#[derive(Debug, StructOpt)]
pub struct CliProduceDeals<F, G, H> {
    #[structopt(skip)]
    _pd: PhantomData<(F, G, H)>,
}

impl<'a, F, G, H> CliRun<(&'a CliSciRashi<F, G, H>, &'a CliDkg<F, G, H>, &'a Cli<F, G, H>)>
    for CliProduceDeals<F, G, H>
where
    F: PrimeField,
    G: Group<Scalar = F> + GroupEncoding,
{
    fn run(
        &self,
        (csi_rashi, _dkg, cli): (&'a CliSciRashi<F, G, H>, &'a CliDkg<F, G, H>, &'a Cli<F, G, H>),
    ) -> Result<(), AnyError> {
        let key_id = &csi_rashi.key_id;

        if Table::<Session<F, G>>::open(cli.open_storage()?, cli.curve)?
            .get(key_id)?
            .is_some()
        {
            return Err("The deals have already been produced for this key".into())
        }

        let mut rng = cli.rng();
        let input: ProduceDealsInput<F> = serde_yaml::from_reader(std::io::stdin().lock())?;

        let secret = F::random(&mut rng);

        let threshold = input.threshold;
        let mut shamir_xs = input.shamir_xs.into_iter().map(Scalar::into_inner).collect::<Vec<_>>();
        shamir_xs.push(input.own_shamir_x.into_inner());

        let mut shamir_ys = vec![F::ZERO; shamir_xs.len()];

        let commitment = ::csi_rashi_dkg::deal::<F, G, MAX_THRESHOLD>(
            &mut rng,
            threshold,
            &secret,
            &shamir_xs[..],
            &mut shamir_ys[..],
        )?
        .into_iter()
        .take(threshold)
        .map(Point::from)
        .collect::<Vec<_>>();

        let mut shamir_xs = shamir_xs.into_iter().map(Scalar::from).collect::<Vec<_>>();
        let mut shamir_ys = shamir_ys.into_iter().map(Scalar::from).collect::<Vec<_>>();

        let own_shamir_x = shamir_xs.pop().expect("there must be at least one there");
        let own_shamir_y = shamir_ys.pop().expect("there must be at least one there");

        let session = Session {
            shamir_x: own_shamir_x,
            shamir_y: own_shamir_y,
            commitment: commitment.clone(),
        };

        let output = ProduceDealsOutput {
            threshold,
            commitment,
            secret_deals: shamir_xs.into_iter().zip(shamir_ys).collect(),
        };

        Table::<Session<F, G>>::open(cli.open_storage()?, cli.curve)?.insert(key_id, &session)?;

        serde_yaml::to_writer(std::io::stdout().lock(), &json!({ "produce_deals": output }))?;

        Ok(())
    }
}
