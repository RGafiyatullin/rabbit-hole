use core::ops::Neg;
use std::vec::Vec;

use ff::PrimeField;
use group::Group;

fn one_of<F: PrimeField, G: Group<Scalar = F>>(options: &[F], keys: &mut [G], choice: usize) {
    assert_eq!(options.len(), keys.len());

    let mut rng = rand::rngs::OsRng;

    let (a, pa) = ::simplest_ot::sender_init(&mut rng);
    let (r_key, pb) = ::simplest_ot::receiver_choose(&mut rng, &pa, &options[..], choice);

    ::simplest_ot::sender_keys(&a, &pb, options, keys);

    keys.iter().enumerate().for_each(|(idx, s_key)| {
        assert_eq!(s_key == &r_key, idx == choice);
    })
}

fn every_choice_in_one_of<F: PrimeField, G: Group<Scalar = F>>(
    options: impl IntoIterator<Item = impl Into<F>>,
) {
    let options = options.into_iter().map(Into::into).collect::<Vec<_>>();
    let mut keys = vec![G::identity(); options.len()];

    for choice in 0..options.len() {
        keys.iter_mut().for_each(|k| *k = G::identity());

        one_of(&options[..], &mut keys[..], choice);
    }
}

#[test]
fn one_of_items_plus_or_minus_one_secp256k1() {
    type F = k256::Scalar;
    type G = k256::ProjectivePoint;

    let minus_one = F::ONE.neg();
    let plus_one = F::ONE;
    every_choice_in_one_of::<F, G>([minus_one, plus_one]);
}

#[test]
fn one_of_items_plus_or_minus_one_ed25519() {
    type F = curve25519::scalar::Scalar;
    type G = curve25519::edwards::EdwardsPoint;

    let minus_one = F::ONE.neg();
    let plus_one = F::ONE;
    every_choice_in_one_of::<F, G>([minus_one, plus_one]);
}

#[test]
fn one_of_items_plus_or_minus_one_ristretto25519() {
    type F = curve25519::scalar::Scalar;
    type G = curve25519::ristretto::RistrettoPoint;

    let minus_one = F::ONE.neg();
    let plus_one = F::ONE;
    every_choice_in_one_of::<F, G>([minus_one, plus_one]);
}

#[test]
fn one_of_items_zero_through_255_secp256k1() {
    type F = k256::Scalar;
    type G = k256::ProjectivePoint;

    every_choice_in_one_of::<F, G>(0u64..255);
}

#[test]
fn one_of_items_zero_through_255_ed25519() {
    type F = curve25519::scalar::Scalar;
    type G = curve25519::edwards::EdwardsPoint;

    every_choice_in_one_of::<F, G>(0u64..255);
}

#[test]
fn one_of_items_zero_through_255_ristretto25519() {
    type F = curve25519::scalar::Scalar;
    type G = curve25519::ristretto::RistrettoPoint;

    every_choice_in_one_of::<F, G>(0u64..255);
}
