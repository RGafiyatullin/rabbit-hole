use ff::{Field, PrimeField};
use group::Group;
use rand::RngCore;

use ::shamir_sss::*;

use super::*;

pub fn basic_impl<F, G>(mut rng: impl RngCore)
where
    F: Field + PrimeField,
    G: Group<Scalar = F>,
{
    const PARTIES: usize = 10;
    const THRESHOLD: usize = 3;

    let g = G::generator();

    let y_0 = F::random(&mut rng);
    let p = g * y_0;

    let mut scheme = [F::default(); 3];
    scheme.init_from_secret(&y_0, &mut rng);

    let xs = std::iter::repeat_with(|| F::random(&mut rng)).take(PARTIES).collect::<Vec<_>>();
    let ys = xs.iter().copied().map(|x| scheme.issue_share(x)).collect::<Vec<_>>();

    for i in 0..PARTIES {
        let xs: [_; THRESHOLD] = core::array::from_fn(|j| xs[(i + j) % PARTIES]);
        let ys: [_; THRESHOLD] = core::array::from_fn(|j| ys[(i + j) % PARTIES]);
        let ls: [_; THRESHOLD] = core::array::from_fn(|j| xs.lagrange_coefficient_at(j, F::ZERO));

        let ps: [_; THRESHOLD] = core::array::from_fn(|j| g * (ys[j] * ls[j]));

        assert_eq!(ys.iter().zip(ls.iter()).map(|(&y, &l)| y * l).sum::<F>(), y_0);
        assert_eq!(ps.into_iter().sum::<G>(), p);
    }
}

#[test]
fn basic() {
    basic_impl::<Scalar, Point>(&mut rand::rngs::OsRng);
}
