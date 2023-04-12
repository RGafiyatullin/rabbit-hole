use ff::PrimeField;
use group::Group;
use rand::RngCore;

pub fn sender_init<F, G, const K: usize>(
    mut rng: impl RngCore,
    delta: &mut [F],
    ot_a: &mut [F],
    ot_pa: &mut [G],
) where F: PrimeField, G: Group<Scalar = F> {
    let ell = (F::NUM_BITS as usize) + K;
    assert_eq!(delta.len(), ell);
    assert_eq!(ot_a.len(), ell);
    assert_eq!(ot_pa.len(), ell);

    delta.iter_mut().for_each(|d| *d = F::random(&mut rng));

    ot_a.iter_mut().zip(ot_pa.iter_mut())
        .for_each(|(a, pa)| {
            (*a, *pa) = simplest_ot::sender_init(&mut rng);
        });
    
}


