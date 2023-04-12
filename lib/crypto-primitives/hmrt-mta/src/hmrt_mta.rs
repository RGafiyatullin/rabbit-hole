use ff::PrimeField;
use group::Group;
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
    ot_pa: &[G],
    ot_pb: &mut [G],
    ot_rkey: &mut [G],
    t: &mut [F],
) where
    F: PrimeField,
    G: Group<Scalar = F>,
{
    let ell = (F::NUM_BITS as usize) + K;
    assert_eq!(ot_pa.len(), ell);
    assert_eq!(t.len(), ell);

    let options = [F::ONE.neg(), F::ONE];

    ot_pa
        .iter()
        .zip(ot_pb.iter_mut())
        .zip(ot_rkey.iter_mut())
        .zip(t.iter_mut())
        .for_each(|(((pa, pb), rkey), t)| {
            let choice = F::random(&mut rng).is_even().unwrap_u8() as usize;
            (*rkey, *pb) = simplest_ot::receiver_choose(&mut rng, pa, &options[..], choice);
        });
}

pub fn sender_ot_reply<F, G, const K: usize>(
    secret_mult_share: &F,

    delta: &[F],
    ot_a: &[F],
    ot_pb: &[G],

    encrypted: &mut [[F; 2]],
    f_encrypt: impl Fn(&F, &G) -> F,
) where
    F: PrimeField,
    G: Group<Scalar = F>,
{
    let ell = (F::NUM_BITS as usize) + K;

    assert_eq!(delta.len(), ell);
    assert_eq!(ot_a.len(), ell);
    assert_eq!(ot_pb.len(), ell);
    assert_eq!(encrypted.len(), ell);

    let options = [F::ONE.neg(), F::ONE];
    let mut keys = [G::identity(), G::identity()];

    delta
        .iter()
        .zip(ot_a.iter())
        .zip(ot_pb.iter())
        .zip(encrypted.iter_mut())
        .for_each(|(((d, ot_a), ot_pb), encrypted)| {
            simplest_ot::sender_keys(ot_a, ot_pb, &options[..], &mut keys);

            let options = [secret_mult_share.neg() + d, *secret_mult_share + d];

            encrypted[0] = f_encrypt(&options[0], &keys[0]);
            encrypted[1] = f_encrypt(&options[1], &keys[1]);
        });
}
