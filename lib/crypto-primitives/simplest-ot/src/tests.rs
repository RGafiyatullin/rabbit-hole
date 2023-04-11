use ff::PrimeField;
use group::Group;

use crate::*;

fn one_of_n<F: PrimeField, G: Group<Scalar = F>, const SIZE: usize>(choice: usize) {
    let mut rng = rand::rngs::OsRng;

    let (a, pa) = sender_init(&mut rng);

    let (r_key, pb) = receiver_choose(&mut rng, &pa, &(choice as u64).into());

    let mut keys = [G::identity(); SIZE];
    sender_keys(&a, &pb, &mut keys);

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
fn one_of_2_secp256k1() {
    one_of_n_every_choice::<k256::Scalar, k256::ProjectivePoint, 2>();
}

#[test]
fn one_of_2_ed25519() {
    one_of_n_every_choice::<curve25519::scalar::Scalar, curve25519::edwards::EdwardsPoint, 2>();
}

#[test]
fn one_of_2_ristretto25519() {
    one_of_n_every_choice::<curve25519::scalar::Scalar, curve25519::ristretto::RistrettoPoint, 2>();
}

#[test]
fn one_of_3_secp256k1() {
    one_of_n_every_choice::<k256::Scalar, k256::ProjectivePoint, 3>();
}

#[test]
fn one_of_3_ed25519() {
    one_of_n_every_choice::<curve25519::scalar::Scalar, curve25519::edwards::EdwardsPoint, 3>();
}

#[test]
fn one_of_3_ristretto25519() {
    one_of_n_every_choice::<curve25519::scalar::Scalar, curve25519::ristretto::RistrettoPoint, 3>();
}

#[test]
fn one_of_64_secp256k1() {
    one_of_n_every_choice::<k256::Scalar, k256::ProjectivePoint, 64>();
}

#[test]
fn one_of_64_ed25519() {
    one_of_n_every_choice::<curve25519::scalar::Scalar, curve25519::edwards::EdwardsPoint, 64>();
}

#[test]
fn one_of_64_ristretto25519() {
    one_of_n_every_choice::<curve25519::scalar::Scalar, curve25519::ristretto::RistrettoPoint, 64>(
    );
}
