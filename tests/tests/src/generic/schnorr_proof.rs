use ff::Field;
use group::Group;
use rand::RngCore;

use ::schnorr_proof::*;

use super::*;

fn basic_impl<F, G>(mut rng: impl RngCore)
where
    F: Field,
    G: Group<Scalar = F>,
{
    const N: usize = 10;

    let g = G::generator();

    let xs: [_; N] = core::array::from_fn(|_| F::random(&mut rng));
    let ys: [_; N] = xs.map(|x| g * x);
    let cs: [_; N] = core::array::from_fn(|_| F::random(&mut rng));

    let ps: [[_; N]; N] = core::array::from_fn(|prover_idx| {
        core::array::from_fn(|challenge_idx| {
            let k = F::random(&mut rng);
            prove(g, &xs[prover_idx], &k, cs[challenge_idx])
        })
    });

    for correct_prover_idx in 0..N {
        for correct_challenge_idx in 0..N {
            let (s, r) = ps[correct_prover_idx][correct_challenge_idx];

            for prover_idx in 0..N {
                for challenge_idx in 0..N {
                    let correct =
                        prover_idx == correct_prover_idx && challenge_idx == correct_challenge_idx;
                    assert_eq!(correct, verify(g, ys[prover_idx], cs[challenge_idx], s, r));
                }
            }
        }
    }
}

#[test]
fn basic() {
    basic_impl::<Scalar, Point>(&mut rand::rngs::OsRng);
}
