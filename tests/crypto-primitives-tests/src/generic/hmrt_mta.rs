use digest::Digest;
use ff::PrimeField;
use group::{Group, GroupEncoding};

use super::{Point, Scalar};

fn basic_impl<F: PrimeField, G: Group<Scalar = F> + GroupEncoding, const L: usize>() {
    let mut rng = rand::rngs::OsRng;

    let mta_m1 = F::random(&mut rng);
    let mta_m2 = F::random(&mut rng);

    let mut delta = [F::ZERO; L];
    let mut ot_a = [F::ZERO; L];
    let mut ot_pa = [G::identity(); L];

    hmrt_mta::sender_init::<F, G, L>(&mut rng, delta.as_mut(), ot_a.as_mut(), ot_pa.as_mut());

    let mut shared = [F::ZERO; L];

    let mut ot_pb = [G::identity(); L];
    let mut ot_rkey = [G::identity(); L];
    let mut t = [F::ZERO; L];

    hmrt_mta::receiver_ot_choose::<F, G, L>(
        &mut rng,
        &mta_m2,
        ot_pa.as_ref(),
        ot_pb.as_mut(),
        ot_rkey.as_mut(),
        t.as_mut(),
        shared.as_mut(),
    );

    let mut encrypted = [[F::ZERO, F::ZERO]; L];
    hmrt_mta::sender_ot_reply::<F, G, _, L>(
        &mta_m1,
        delta.as_ref(),
        ot_a.as_ref(),
        ot_pb.as_ref(),
        encrypted.as_mut(),
        encrypt::<F, G>,
    );

    let mta_a1 = hmrt_mta::sender_additive_share::<F, L>(shared.as_ref(), delta.as_ref());

    let mta_a2 = hmrt_mta::receiver_additive_share::<F, G, _, L>(
        shared.as_ref(),
        encrypted.as_ref(),
        t.as_ref(),
        ot_rkey.as_ref(),
        decrypt::<F, G>,
    );

    assert_eq!(mta_a1 + mta_a2, mta_m1 * mta_m2);
}

type H = sha3::Sha3_256;

fn encrypt<F, G>(key: &G, n: &F) -> F
where
    F: PrimeField,
    G: GroupEncoding,
{
    let k: F = utils::bytes_to_scalar(H::digest(key.to_bytes()).as_ref());
    *n + k
}
fn decrypt<F, G>(key: &G, n: &F) -> F
where
    F: PrimeField,
    G: GroupEncoding,
{
    let k: F = utils::bytes_to_scalar(H::digest(key.to_bytes()).as_ref());
    *n - k
}

#[test]
fn basic() {
    basic_impl::<Scalar, Point, 256>();
}

fn encrypt_decrypt_impl<F, G>()
where
    F: PrimeField,
    G: Group<Scalar = F> + GroupEncoding,
{
    let mut rng = rand::rngs::OsRng;

    let original = F::random(&mut rng);
    let key = G::generator() * F::random(&mut rng);

    let encrypted = encrypt::<F, G>(&key, &original);
    let decrypted = decrypt::<F, G>(&key, &encrypted);

    assert_eq!(original, decrypted);
}

#[test]
fn encrypt_decrypt() {
    encrypt_decrypt_impl::<Scalar, Point>()
}
