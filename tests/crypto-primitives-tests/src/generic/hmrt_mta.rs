use digest::Digest;
use ff::PrimeField;
use group::{Group, GroupEncoding};

use super::{Point, Scalar};

fn basic_impl<F: PrimeField, G: Group<Scalar = F> + GroupEncoding, const K: usize>() {
    let mut rng = rand::rngs::OsRng;

    let ell = F::NUM_BITS as usize + K;

    let mta_m1 = F::random(&mut rng);
    let mta_m2 = F::random(&mut rng);

    let mut delta = vec![F::ZERO; ell];
    let mut ot_a = vec![F::ZERO; ell];
    let mut ot_pa = vec![G::identity(); ell];

    hmrt_mta::sender_init::<F, G, K>(&mut rng, delta.as_mut(), ot_a.as_mut(), ot_pa.as_mut());

    let mut ot_pb = vec![G::identity(); ell];
    let mut ot_rkey = vec![G::identity(); ell];
    let mut t = vec![F::ZERO; ell];

    hmrt_mta::receiver_ot_choose::<F, G, K>(
        &mut rng,
        ot_pa.as_ref(),
        ot_pb.as_mut(),
        ot_rkey.as_mut(),
        t.as_mut(),
    );

    let mut encrypted = vec![[F::ZERO, F::ZERO]; ell];
    hmrt_mta::sender_ot_reply::<F, G, K>(
        &mta_m1,
        delta.as_ref(),
        ot_a.as_ref(),
        ot_pb.as_ref(),
        encrypted.as_mut(),
        encrypt::<F, G, sha3::Sha3_256>,
    );
}

fn encrypt<F, G, H>(n: &F, key: &G) -> F
where
    F: PrimeField,
    G: GroupEncoding,
    H: Digest,
{
    let h = H::new().chain_update(key.to_bytes()).finalize();
    let k = utils::bytes_to_scalar(h.as_ref());
    *n + k
}
fn decrypt<F, G, H>(n: &F, key: &G) -> F
where
    F: PrimeField,
    G: GroupEncoding,
    H: Digest,
{
    let h = H::new().chain_update(key.to_bytes()).finalize();
    let k = utils::bytes_to_scalar(h.as_ref());
    *n - k
}

const K_NONE: usize = 0;
const K_SOME: usize = 10;
const K_MANY: usize = 255;

#[test]
fn basic_k_none() {
    basic_impl::<Scalar, Point, K_NONE>();
}
#[test]
fn basic_k_some() {
    basic_impl::<Scalar, Point, K_SOME>();
}

#[test]
fn basic_k_many() {
    basic_impl::<Scalar, Point, K_MANY>();
}
