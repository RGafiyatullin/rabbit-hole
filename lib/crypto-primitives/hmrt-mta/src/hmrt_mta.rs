use ff::{Field, PrimeField};
use group::{Group, GroupEncoding};
use rand::RngCore;

pub fn sender_init<F, G, const K: usize>(
    mut rng: impl RngCore,
    delta: &mut [F],
    ot_a: &mut [F],
    ot_pa: &mut [G],
) where
    F: PrimeField,
    G: Group<Scalar = F>,
{
    let ell = (F::NUM_BITS as usize) + K;
    assert_eq!(delta.len(), ell);
    assert_eq!(ot_a.len(), ell);
    assert_eq!(ot_pa.len(), ell);

    delta.iter_mut().for_each(|d| *d = F::random(&mut rng));

    ot_a.iter_mut().zip(ot_pa.iter_mut()).for_each(|(a, pa)| {
        (*a, *pa) = simplest_ot::sender_init(&mut rng);
    });
}

pub fn receiver_ot_choose<F, G, const K: usize>(
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
    let ell = (F::NUM_BITS as usize) + K;
    assert_eq!(ot_pa.len(), ell);
    assert_eq!(ot_pb.len(), ell);
    assert_eq!(ot_rkey.len(), ell);
    assert_eq!(shared.len(), ell);
    assert_eq!(t.len(), ell);

    let options = options_neg_or_pos::<F>();

    ot_pa
        .iter()
        .zip(ot_pb.iter_mut())
        .zip(ot_rkey.iter_mut())
        .zip(t.iter_mut())
        .for_each(|(((pa, pb), rkey), t)| {
            let choice = F::random(&mut rng).is_even().unwrap_u8() as usize;
            (*rkey, *pb) = simplest_ot::receiver_choose(&mut rng, pa, &options[..], choice);
            *t = options[choice];
        });

    shared.iter_mut().skip(1).for_each(|v| *v = F::random(&mut rng));
    let head =
        *secret_mult_share - shared.iter().zip(t.iter()).map(|(v, t)| *v * t).skip(1).sum::<F>();
    shared.iter_mut().zip(t.iter()).take(1).for_each(|(v, t)| *v = head * t);

    assert_eq!(shared.iter().zip(t.iter()).map(|(v, t)| *v * t).sum::<F>(), *secret_mult_share);
}

pub fn sender_ot_reply<F, G, const K: usize>(
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
    let ell = (F::NUM_BITS as usize) + K;

    assert_eq!(delta.len(), ell);
    assert_eq!(ot_a.len(), ell);
    assert_eq!(ot_pb.len(), ell);
    assert_eq!(encrypted.len(), ell);

    let options = options_neg_or_pos::<F>();
    let mut keys = [G::identity(), G::identity()];

    delta
        .iter()
        .zip(ot_a.iter())
        .zip(ot_pb.iter())
        .zip(encrypted.iter_mut())
        .for_each(|(((d, ot_a), ot_pb), encrypted)| {
            simplest_ot::sender_keys(ot_a, ot_pb, &options[..], &mut keys);

            let options = options.map(|t| t * secret_mult_share + d);

            encrypted[0] = f_encrypt(&keys[0], &options[0]);
            encrypted[1] = f_encrypt(&keys[1], &options[1]);
        });
}

pub fn sender_additive_share<F, const K: usize>(shared: &[F], delta: &[F]) -> F
where
    F: PrimeField,
{
    let ell = (F::NUM_BITS as usize) + K;
    assert_eq!(delta.len(), ell);
    assert_eq!(shared.len(), ell);

    delta.iter().zip(shared.iter()).map(|(d, s)| *d + s).sum::<F>().neg()
}

pub fn receiver_additive_share<F, G, const K: usize>(
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
    let ell = (F::NUM_BITS as usize) + K;
    assert_eq!(shared.len(), ell);
    assert_eq!(encrypted.len(), ell);
    assert_eq!(ot_rkey.len(), ell);
    assert_eq!(t.len(), ell);

    let z = encrypted.iter().zip(t.iter()).zip(ot_rkey.iter()).map(|((encrypted, t), key)| {
        let choice = if *t == F::ONE { 1 } else { 0 };
        let encrypted = &encrypted[choice];

        f_decrypt(key, encrypted)
    });

    shared.iter().zip(z).map(|(s, z)| z * s).sum::<F>()
}

fn options_neg_or_pos<F: Field>() -> [F; 2] {
    [F::ONE.neg(), F::ONE]
}
