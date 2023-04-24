use std::collections::HashMap;

use digest::Digest;
use ff::PrimeField;
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
fn run_secp256k1_in_tmp_dir() {
    run_various_configurations(CurveSelect::Secp256k1, "k1", true)
}

#[test]
fn run_ed25519_in_tmp_dir() {
    run_various_configurations(CurveSelect::Ed25519, "ed", true)
}

#[test]
fn run_ristretto25519_in_tmp_dir() {
    run_various_configurations(CurveSelect::Ristretto25519, "ri", true)
}

#[test]
#[ignore]
fn run_secp256k1_in_std_dir() {
    run_various_configurations(CurveSelect::Secp256k1, "k1", false)
}

#[test]
#[ignore]
fn run_ed25519_in_std_dir() {
    run_various_configurations(CurveSelect::Ed25519, "ed", false)
}

#[test]
#[ignore]
fn run_ristretto25519_in_std_dir() {
    run_various_configurations(CurveSelect::Ristretto25519, "ri", false)
}

fn run_various_configurations(curve: CurveSelect, prefix: &str, use_temp_dir: bool) {
    let tmp: tempfile::TempDir;
    let storage_override = if use_temp_dir {
        tmp = tempfile::tempdir().expect("Tempdir");
        let tmp = tmp.path();
        Some(tmp.to_str().expect("to-str"))
    } else {
        None
    };

    for p in 2..=4 {
        for t in 2..=p {
            run_untyped(
                curve,
                format!("{}-{}-of-{}", prefix, t, p).as_str(),
                2,
                4,
                storage_override,
            );
        }
    }
}

fn run_untyped(
    curve: CurveSelect,
    key_prefix: &str,
    threshold: usize,
    parties_count: usize,
    override_storage: Option<&str>,
) {
    specialize_call!(run_typed, (curve, key_prefix, threshold, parties_count, override_storage), curve, [
        (CurveSelect::Secp256k1 => k256::Scalar),
        (CurveSelect::Ed25519 | CurveSelect::Ristretto25519 => curve25519::scalar::Scalar),
    ]).expect("Unsupported curve")
}

fn run_typed<F: PrimeField>(
    curve: CurveSelect,
    key_prefix: &str,
    threshold: usize,
    parties_count: usize,
    override_storage: Option<&str>,
) {
    assert!(threshold <= parties_count);

    let mut rng = rand::rngs::OsRng;

    let storage_arg = override_storage
        .map(|p| format!("--storage-path {} ", p))
        .unwrap_or("".to_owned());
    let shamir_xs = (0..parties_count)
        .map(|idx| utils::bytes_to_scalar::<F>(sha3::Sha3_256::digest(idx.to_ne_bytes()).as_ref()))
        .map(|x| Scalar::from_value(curve, x))
        .collect::<Vec<_>>();

    let key_ids = (0..parties_count)
        .map(|idx| format!("{}:{}", key_prefix, idx))
        .collect::<Vec<_>>();

    let mut deal_outputs: Vec<DealOutput> = vec![];

    for party_idx in 0..parties_count {
        let key_id = key_ids[party_idx].as_str();

        let io = TestIO::from_yaml_stdin(json!({
            "threshold": threshold,
            "this": party_idx,
            "shamir_xs": shamir_xs,
        }))
        .expect("make io");

        let cli = Cli::create_safe(args(format!(
            "{}dkg csi-rashi deal --curve {} {}",
            storage_arg, curve, key_id,
        )))
        .expect("args error");

        assert_eq!(cli.run((&mut rng, &io)).expect("cli-run"), 0);

        let deal_output = io.stdout_as_yaml::<DealOutput>().expect("io:de");
        deal_outputs.push(deal_output);
    }

    for party_idx in 0..parties_count {
        let key_id = key_ids[party_idx].as_str();

        let this_party_x = &shamir_xs[party_idx];
        let mut commitments = HashMap::<Scalar, Vec<Point>>::new();
        let mut deals = HashMap::<Scalar, Scalar>::new();

        for other_party_idx in (0..parties_count).filter(|&i| i != party_idx) {
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
        let cli =
            Cli::create_safe(args(format!("{}dkg csi-rashi aggregate {}", storage_arg, key_id,)))
                .expect("args error");

        assert_eq!(cli.run((&mut rng, &io)).expect("cli-run"), 0);
    }

    let mut public_keys = vec![];
    for party_idx in 0..parties_count {
        let key_id = key_ids[party_idx].as_str();

        let io = TestIO::from_empty_input();
        let cli = Cli::create_safe(args(format!("{}keys get {}", storage_arg, key_id,)))
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
