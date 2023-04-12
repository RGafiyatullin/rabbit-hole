use ff::PrimeField;
use group::{Group, GroupEncoding};
use rand::RngCore;

use crate::hmrt_mta;

pub fn sender_init<F, G>(
    rng: impl RngCore,
    delta: &mut [F],
    ot_a: &mut [F],
    ot_pa: &mut [G],
) where
    F: PrimeField,
    G: Group<Scalar = F>,
{
    hmrt_mta::sender_init::<F, G, 0>(rng, delta, ot_a, ot_pa)
}

pub fn receiver_ot_choose<F, G>(
    rng: impl RngCore,
    secret_mult_share: &F,

    ot_pa: &[G],
    ot_pb: &mut [G],
    ot_rkey: &mut [G],
    t: &mut [F],
    shared: &mut [F],
) where
    F: PrimeField,
    G: Group<Scalar = F>,
{
    hmrt_mta::receiver_ot_choose::<F, G, 0>(rng, secret_mult_share, ot_pa, ot_pb, ot_rkey, t, shared)
}

pub fn sender_ot_reply<F, G>(
    secret_mult_share: &F,

    delta: &[F],
    ot_a: &[F],
    ot_pb: &[G],

    encrypted: &mut [[F; 2]],
    f_encrypt: impl Fn(&G, &F) -> F,
) where
    F: PrimeField,
    G: Group<Scalar = F>,
{
    hmrt_mta::sender_ot_reply::<F, G, 0>(secret_mult_share, delta, ot_a, ot_pb, encrypted, f_encrypt)
}

pub fn sender_additive_share<F>(shared: &[F], delta: &[F]) -> F
where
    F: PrimeField,
{
    hmrt_mta::sender_additive_share::<F, 0>(shared, delta)
}

pub fn receiver_additive_share<F, G>(
    shared: &[F],
    encrypted: &[[F; 2]],
    t: &[F],
    ot_rkey: &[G],
    f_decrypt: impl Fn(&G, &F) -> F,
) -> F
where
    F: PrimeField,
    G: GroupEncoding,
{
    hmrt_mta::receiver_additive_share::<F, G, 0>(shared, encrypted, t, ot_rkey, f_decrypt)
}
