use digest::Digest;
use ff::PrimeField;
use group::{Group, GroupEncoding};

use super::{Point, Scalar};

fn basic_impl<F: PrimeField, G: Group<Scalar = F> + GroupEncoding>() {
    let mut rng = rand::rngs::OsRng;

    let ell = F::NUM_BITS as usize;

    let mta_m1 = F::random(&mut rng);
    let mta_m2 = F::random(&mut rng);

    let mut delta = vec![F::ZERO; ell];
    let mut ot_a = vec![F::ZERO; ell];
    let mut ot_pa = vec![G::identity(); ell];

    hmrt_mta::sender_init::<F, G>(&mut rng, delta.as_mut(), ot_a.as_mut(), ot_pa.as_mut());

    let mut shared = vec![F::ZERO; ell];

    let mut ot_pb = vec![G::identity(); ell];
    let mut ot_rkey = vec![G::identity(); ell];
    let mut t = vec![F::ZERO; ell];

    hmrt_mta::receiver_ot_choose::<F, G>(
        &mut rng,
        &mta_m2,
        ot_pa.as_ref(),
        ot_pb.as_mut(),
        ot_rkey.as_mut(),
        t.as_mut(),
        shared.as_mut(),
    );

    let mut encrypted = vec![[F::ZERO, F::ZERO]; ell];
    hmrt_mta::sender_ot_reply::<F, G>(
        &mta_m1,
        delta.as_ref(),
        ot_a.as_ref(),
        ot_pb.as_ref(),
        encrypted.as_mut(),
        encrypt::<F, G, sha3::Sha3_256>,
    );

    let mta_a1 = hmrt_mta::sender_additive_share::<F>(shared.as_ref(), delta.as_ref());

    let mta_a2 = hmrt_mta::receiver_additive_share::<F, G>(
        shared.as_ref(),
        encrypted.as_ref(),
        t.as_ref(),
        ot_rkey.as_ref(),
        decrypt::<F, G, sha3::Sha3_256>,
    );

    assert_eq!(mta_a1 + mta_a2, mta_m1 * mta_m2);
}

fn encrypt<F, G, H>(key: &G, n: &F) -> F
where
    F: PrimeField,
    G: GroupEncoding,
    H: Digest,
{
    let h = H::new().chain_update(key.to_bytes()).finalize();
    let k: F = utils::bytes_to_scalar(h.as_ref());
    *n + k
}
fn decrypt<F, G, H>(key: &G, n: &F) -> F
where
    F: PrimeField,
    G: GroupEncoding,
    H: Digest,
{
    let h = H::new().chain_update(key.to_bytes()).finalize();
    let k: F = utils::bytes_to_scalar(h.as_ref());
    *n - k
}

#[test]
fn basic_k_none() {
    basic_impl::<Scalar, Point>();
}
