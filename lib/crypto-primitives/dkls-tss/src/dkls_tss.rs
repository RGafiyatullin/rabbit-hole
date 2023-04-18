// use ff::Field;
// use group::Group;
// use rand::RngCore;

// pub fn bob_init<F, G>(mut rng: impl RngCore, k_b: &mut [F], d_b: &mut [G])
// where F: Field, G: Group<Scalar = F>
// {
//     assert_eq!(k_b.len(), d_b.len());

//     let g = G::generator();

//     for (k_b, d_b) in k_b.iter_mut().zip(d_b.iter_mut()) {
//         *k_b = F::random(&mut rng);
//         *d_b = g * *k_b;
//     }
// }

// pub fn alice_init<F, G>(mut rng: impl RngCore, phi: &mut [F], k_a: &mut [F], r: &mut [G]) where
// F: Field, G: Group<Scalar = F> {

// }
