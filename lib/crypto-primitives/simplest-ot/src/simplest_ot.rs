use ff::PrimeField;
use group::Group;
use rand::RngCore;

pub fn sender_init<F, G>(rng: impl RngCore) -> (F, G)
where
    F: PrimeField,
    G: Group<Scalar = F>,
{
    let a = F::random(rng);
    let pa = G::generator() * a;

    (a, pa)
}

pub fn receiver_choose<F, G>(rng: impl RngCore, pa: &G, options: &[F], choice: usize) -> (G, G)
where
    F: PrimeField,
    G: Group<Scalar = F>,
{
    let b = F::random(rng);
    let offset = *pa * &options[choice];
    let pb = offset + G::generator() * b;
    let key = *pa * b;

    (key, pb)
}

pub fn sender_keys<F, G>(a: &F, pb: &G, options: &[F], keys: &mut [G])
where
    F: PrimeField,
    G: Group<Scalar = F>,
{
    assert_eq!(options.len(), keys.len());

    let pa = G::generator() * a;
    options.iter().zip(keys.iter_mut()).for_each(|(choice, key)| {
        let offset = pa * choice;
        *key = (*pb - offset) * a;
    });
}
