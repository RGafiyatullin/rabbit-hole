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

pub fn receiver_choose<F, G>(rng: impl RngCore, pa: &G, choice: &F) -> (G, G)
where
    F: PrimeField,
    G: Group<Scalar = F>,
{
    let b = F::random(rng);
    let offset = *pa * choice;
    let pb = offset + G::generator() * b;
    let key = *pa * b;

    (key, pb)
}

pub fn sender_keys<F, G>(a: &F, pb: &G, keys: &mut [G])
where
    F: PrimeField,
    G: Group<Scalar = F>,
{
    let pa = G::generator() * a;
    keys.iter_mut().enumerate().for_each(|(choice, key)| {
        let offset = pa * F::from(choice as u64);
        *key = (*pb - offset) * a;
    });
}
