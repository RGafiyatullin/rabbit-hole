use digest::Digest;
use ff::{Field, PrimeField};
use group::{Group, GroupEncoding};
use rand::RngCore;

type H = sha3::Sha3_256;

fn encrypt<F, G>(key: &G, n: &F) -> F
where
    F: PrimeField,
    G: GroupEncoding,
{
    let k: F = utils::bytes_to_scalar(H::new().chain_update(key.to_bytes()).finalize().as_ref());
    *n + k
}
fn decrypt<F, G>(key: &G, n: &F) -> F
where
    F: PrimeField,
    G: GroupEncoding,
{
    let k: F = utils::bytes_to_scalar(H::new().chain_update(key.to_bytes()).finalize().as_ref());
    *n - k
}

fn demo<F, G, const L: usize>()
where
    F: PrimeField,
    G: Group<Scalar = F> + GroupEncoding,
{
    let mut rng = rand::rngs::OsRng;

    let options = [F::ONE.neg(), F::ONE];

    let m1 = F::random(&mut rng);
    let m2 = F::random(&mut rng);

    let mut delta = [F::ZERO; L];
    let mut a = [F::ZERO; L];
    let mut pa = [G::identity(); L];
    let mut zc = [[F::ZERO; 2]; L];
    let mut zc_cheat = [[F::ZERO; 2]; L];

    let mut pb = [G::identity(); L];
    let mut rkey = [G::identity(); L];
    let mut t = [0usize; L];
    let mut z = [F::ZERO; L];
    let mut v = [F::ZERO; L];

    {
        let (a, pa): (F, G) = simplest_ot::sender_init(&mut rng);
        let (rkey, pb): (G, G) = simplest_ot::receiver_choose(&mut rng, &pa, options.as_ref(), 0);
        let mut keys = [G::identity(); 2];
        simplest_ot::sender_keys(&a, &pb, options.as_ref(), &mut keys);

        assert_eq!(keys[0], rkey);
    }

    // P1:
    fill_random(&mut rng, delta.as_mut());

    for i in 0..L {
        (a[i], pa[i]) = simplest_ot::sender_init::<F, G>(&mut rng);
    }

    // P2:
    for i in 0..L {
        t[i] = rng.next_u32() as usize % 2;
        (rkey[i], pb[i]) =
            simplest_ot::receiver_choose::<F, G>(&mut rng, &pa[i], options.as_ref(), t[i]);
    }

    // P1:
    for i in 0..L {
        let mut keys = [G::identity(); 2];
        simplest_ot::sender_keys(&a[i], &pb[i], options.as_ref(), keys.as_mut());

        zc[i][0] = encrypt(&keys[0], &(options[0] * m1 + delta[i]));
        zc[i][1] = encrypt(&keys[1], &(options[1] * m1 + delta[i]));

        zc_cheat[i][0] = options[0] * m1 + delta[i];
        zc_cheat[i][1] = options[1] * m1 + delta[i];
    }

    // P2:
    for i in 0..L {
        z[i] = decrypt(&rkey[i], &zc[i][t[i]]);

        assert_eq!(z[i], zc_cheat[i][t[i]]);
    }
    fill_random(&mut rng, &mut v);
    let current = corner_brackets_product(v.iter().copied(), (0..L).map(|i| options[t[i]]));
    v[0] += (m2 - current) * options[t[0]];

    assert_eq!(corner_brackets_product(v.iter().copied(), (0..L).map(|i| options[t[i]])), m2);

    // P1:
    let a1 = corner_brackets_product(v, delta).neg();
    // P2:
    let a2 = corner_brackets_product(v, z);

    assert_eq!(m1 * m2, a1 + a2);
}

fn fill_random<F: Field>(mut rng: impl RngCore, out: &mut [F]) {
    out.iter_mut().for_each(|v| *v = F::random(&mut rng));
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

#[test]
fn demo_secp256k1() {
    demo::<k256::Scalar, k256::ProjectivePoint, { k256::Scalar::NUM_BITS as usize }>()
}
