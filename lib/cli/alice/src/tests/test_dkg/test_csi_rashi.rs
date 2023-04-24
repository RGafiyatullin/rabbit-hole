use std::collections::HashMap;

use serde::Deserialize;
use serde_json::json;

use common_interop::curve_select::CurveSelect;
use common_interop::types::{Point, Scalar};

use crate::cli::*;
use crate::data::key::Key;
use crate::tests::cli_utils::args;
use crate::tests::io_utils::TestIO;

#[derive(Debug, Deserialize)]
struct DealOutput {
    deals: HashMap<Scalar, Scalar>,
    commitment: Vec<Point>,
}

#[test]
fn run_2_of_3_k256() {
    run_2_of_3(CurveSelect::Secp256k1)
}

#[test]
fn run_2_of_3_ed25519() {
    run_2_of_3(CurveSelect::Ed25519)
}

#[test]
fn run_2_of_3_ristretto25519() {
    run_2_of_3(CurveSelect::Ristretto25519)
}

fn run_2_of_3(curve: CurveSelect) {
    let mut rng = rand::rngs::OsRng;

    let storage = [
        tempfile::tempdir().expect("tempdir"),
        tempfile::tempdir().expect("tempdir"),
        tempfile::tempdir().expect("tempdir"),
    ];

    let shamir_xs = [
        Scalar::from_hex(curve, "0000000000000000000000000000000000000000000000000000000000010101"),
        Scalar::from_hex(curve, "0000000000000000000000000000000000000000000000000000000000020202"),
        Scalar::from_hex(curve, "0000000000000000000000000000000000000000000000000000000000030203"),
    ];

    let key_id = "the-key";

    let mut deal_outputs: Vec<DealOutput> = vec![];

    for party_idx in 0..3 {
        let io = TestIO::from_yaml_stdin(json!({
            "threshold": 2,
            "this": party_idx,
            "shamir_xs": [
                shamir_xs[0],
                shamir_xs[1],
                shamir_xs[2],
            ]
        }))
        .expect("make io");

        let cli = Cli::create_safe(args(format!(
            "--storage-path {} dkg csi-rashi deal --curve {} {}",
            storage[party_idx].path().to_str().unwrap(),
            curve,
            key_id,
        )))
        .expect("args error");

        assert_eq!(cli.run((&mut rng, &io)).expect("cli-run"), 0);

        let deal_output = io.stdout_as_yaml::<DealOutput>().expect("io:de");
        deal_outputs.push(deal_output);
    }

    for party_idx in 0..3 {
        let this_party_x = &shamir_xs[party_idx];
        let mut commitments = HashMap::<Scalar, Vec<Point>>::new();
        let mut deals = HashMap::<Scalar, Scalar>::new();

        for other_party_idx in (0..3).filter(|&i| i != party_idx) {
            let other_party_x = &shamir_xs[other_party_idx];
            commitments
                .insert(other_party_x.clone(), deal_outputs[other_party_idx].commitment.clone());
            deals.insert(
                other_party_x.clone(),
                deal_outputs[other_party_idx].deals.get(this_party_x).unwrap().clone(),
            );
        }

        let io = TestIO::from_yaml_stdin(json!({
            "commitments": commitments,
            "deals": deals,
        }))
        .expect("make io");
        let cli = Cli::create_safe(args(format!(
            "--storage-path {} dkg csi-rashi aggregate {}",
            storage[party_idx].path().to_str().unwrap(),
            key_id,
        )))
        .expect("args error");

        assert_eq!(cli.run((&mut rng, &io)).expect("cli-run"), 0);
    }

    let mut public_keys = vec![];
    for party_idx in 0..3 {
        let io = TestIO::from_empty_input();
        let cli = Cli::create_safe(args(format!(
            "--storage-path {} keys get {}",
            storage[party_idx].path().to_str().unwrap(),
            key_id,
        )))
        .expect("args error");

        assert_eq!(cli.run((&mut rng, &io)).expect("cli-run"), 0);

        let key: Key = io.stdout_as_yaml().expect("io:de");
        let Key::S4Share(s4_share) = key else { panic!("not an s4-share") };
        assert_eq!(s4_share.curve, curve);

        public_keys.push(s4_share.public_key);
    }

    public_keys
        .into_iter()
        .reduce(|left, right| {
            assert_eq!(left, right);
            left
        })
        .expect("there must be something here");
}
