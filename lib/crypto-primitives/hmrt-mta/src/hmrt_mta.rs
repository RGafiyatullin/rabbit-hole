use ff::{Field, PrimeField};
use group::{Group, GroupEncoding};
use rand::RngCore;

pub fn sender_init<F, G, const L: usize>(
    mut rng: impl RngCore,
    delta: &mut [F],
    ot_a: &mut [F],
    ot_pa: &mut [G],
) where
    F: PrimeField,
    G: Group<Scalar = F>,
{
    assert!(L >= F::NUM_BITS as usize);
    assert_eq!(delta.len(), L);
    assert_eq!(ot_a.len(), L);
    assert_eq!(ot_pa.len(), L);

    fill_random(&mut rng, delta);

    for i in 0..L {
        (ot_a[i], ot_pa[i]) = simplest_ot::sender_init(&mut rng);
    }
}

pub fn receiver_ot_choose<F, G, const L: usize>(
    mut rng: impl RngCore,
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
    assert!(L >= F::NUM_BITS as usize);

    assert_eq!(ot_pa.len(), L);
    assert_eq!(ot_pb.len(), L);
    assert_eq!(ot_rkey.len(), L);
    assert_eq!(shared.len(), L);
    assert_eq!(t.len(), L);

    let options = options_neg_or_pos::<F>();

    for i in 0..L {
        let choice = rng.next_u32() as usize % 2;
        (ot_rkey[i], ot_pb[i]) =
            simplest_ot::receiver_choose(&mut rng, &ot_pa[i], &options, choice);
        t[i] = options[choice];
    }

    fill_random(&mut rng, shared);
    let attempt: F = corner_brackets_product(shared.iter().copied(), (0..L).map(|i| t[i]));
    shared[0] += t[0] * (*secret_mult_share - attempt);

    assert_eq!(
        corner_brackets_product(shared.iter().copied(), (0..L).map(|i| t[i])),
        *secret_mult_share
    );
}

pub fn sender_ot_reply<F, G, E, const L: usize>(
    secret_mult_share: &F,

    delta: &[F],
    ot_a: &[F],
    ot_pb: &[G],

    encrypted: &mut [[E; 2]],
    f_encrypt: impl Fn(&G, &F) -> E,
) where
    F: PrimeField,
    G: Group<Scalar = F>,
{
    assert!(L >= F::NUM_BITS as usize);

    assert_eq!(delta.len(), L);
    assert_eq!(ot_a.len(), L);
    assert_eq!(ot_pb.len(), L);
    assert_eq!(encrypted.len(), L);

    let options = options_neg_or_pos::<F>();
    let mut keys = [G::identity(); 2];

    for i in 0..L {
        simplest_ot::sender_keys(&ot_a[i], &ot_pb[i], options.as_ref(), keys.as_mut());

        let options = options.map(|t| t * secret_mult_share + delta[i]);
        for choice in [0, 1] {
            encrypted[i][choice] = f_encrypt(&keys[choice], &options[choice]);
        }
    }
}

pub fn sender_additive_share<F, const L: usize>(shared: &[F], delta: &[F]) -> F
where
    F: PrimeField,
{
    assert!(L >= F::NUM_BITS as usize);
    assert_eq!(delta.len(), L);
    assert_eq!(shared.len(), L);

    corner_brackets_product(shared.iter().copied(), delta.iter().copied()).neg()
}

pub fn receiver_additive_share<F, G, E, const L: usize>(
    shared: &[F],
    encrypted: &[[E; 2]],
    t: &[F],
    ot_rkey: &[G],
    f_decrypt: impl Fn(&G, &E) -> F,
) -> F
where
    F: PrimeField,
    G: GroupEncoding,
{
    assert!(L >= F::NUM_BITS as usize);
    assert_eq!(shared.len(), L);
    assert_eq!(encrypted.len(), L);
    assert_eq!(ot_rkey.len(), L);
    assert_eq!(t.len(), L);

    let z = encrypted.iter().zip(t.iter()).zip(ot_rkey.iter()).map(|((encrypted, t), key)| {
        let choice = if *t == F::ONE { 1 } else { 0 };
        let encrypted = &encrypted[choice];

        f_decrypt(key, encrypted)
    });

    corner_brackets_product(shared.iter().copied(), z)
}

fn options_neg_or_pos<F: Field>() -> [F; 2] {
    [F::ONE.neg(), F::ONE]
}

fn corner_brackets_product<F>(
    left: impl IntoIterator<Item = F>,
    right: impl IntoIterator<Item = F>,
) -> F
where
    F: Field,
{
    left.into_iter().zip(right).map(|(l, r)| l * r).sum::<F>()
}

fn fill_random<F: Field>(mut rng: impl RngCore, out: &mut [F]) {
    out.iter_mut().for_each(|v| *v = F::random(&mut rng));
}
