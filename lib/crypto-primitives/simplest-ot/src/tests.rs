use core::ops::Neg;
use std::vec::Vec;

use ff::PrimeField;
use group::Group;

use crate::*;

fn one_of<F: PrimeField, G: Group<Scalar = F>>(options: &[F], keys: &mut [G], choice: usize) {
    assert_eq!(options.len(), keys.len());

    let mut rng = rand::rngs::OsRng;

    let (a, pa) = sender_init(&mut rng);
    let (r_key, pb) = receiver_choose(&mut rng, &pa, &options[..], choice);

    sender_keys(&a, &pb, options, keys);

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

fn one_of_n<F: PrimeField, G: Group<Scalar = F>, const SIZE: usize>(choice: usize) {
    let mut rng = rand::rngs::OsRng;

    let (a, pa) = sender_init(&mut rng);
    let options: [F; SIZE] = core::array::from_fn(|i| (i as u64).into());

    let (r_key, pb) = receiver_choose(&mut rng, &pa, &options[..], choice);

    let mut keys = [G::identity(); SIZE];
    sender_keys(&a, &pb, &options[..], &mut keys);

    keys.iter().enumerate().for_each(|(idx, s_key)| {
        assert_eq!(s_key == &r_key, idx == choice);
    })
}

fn one_of_n_every_choice<F: PrimeField, G: Group<Scalar = F>, const SIZE: usize>() {
    for c in 0..SIZE {
        one_of_n::<F, G, SIZE>(c);
    }
}

#[test]
fn zero_or_one_secp256k1() {
    one_of_n_every_choice::<k256::Scalar, k256::ProjectivePoint, 2>()
}

#[test]
fn zero_or_one_ed25519() {
    one_of_n_every_choice::<curve25519::scalar::Scalar, curve25519::edwards::EdwardsPoint, 2>()
}

#[test]
fn zero_or_one_ristretto25519() {
    one_of_n_every_choice::<curve25519::scalar::Scalar, curve25519::ristretto::RistrettoPoint, 2>()
}

#[test]
fn zero_through_three_secp256k1() {
    one_of_n_every_choice::<k256::Scalar, k256::ProjectivePoint, 3>()
}

#[test]
fn zero_through_three_ed25519() {
    one_of_n_every_choice::<curve25519::scalar::Scalar, curve25519::edwards::EdwardsPoint, 3>()
}

#[test]
fn zero_through_three_ristretto25519() {
    one_of_n_every_choice::<curve25519::scalar::Scalar, curve25519::ristretto::RistrettoPoint, 3>()
}

#[test]
fn zero_through_64_secp256k1() {
    one_of_n_every_choice::<k256::Scalar, k256::ProjectivePoint, 64>()
}

#[test]
fn zero_through_64_ed25519() {
    one_of_n_every_choice::<curve25519::scalar::Scalar, curve25519::edwards::EdwardsPoint, 64>()
}

#[test]
fn zero_through_64_ristretto25519() {
    one_of_n_every_choice::<curve25519::scalar::Scalar, curve25519::ristretto::RistrettoPoint, 64>()
}
