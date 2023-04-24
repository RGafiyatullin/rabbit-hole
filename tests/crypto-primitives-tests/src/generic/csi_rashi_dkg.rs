use ff::PrimeField;
use group::Group;
use rand::RngCore;

use ::csi_rashi_dkg::{aggregate, deal};
use ::shamir_sss::LagrangeCoefficientAt;

use super::*;

fn basic_impl<F, G, const PARTIES: usize, const THRESHOLD: usize>(mut rng: impl RngCore)
where
    F: PrimeField,
    G: Group<Scalar = F>,
{
    let g = G::generator();

    let shamir_xs: [_; PARTIES] = core::array::from_fn(|_| F::random(&mut rng));
    let mut shamir_ys = [[F::ZERO; PARTIES]; PARTIES];

    let vss_commitments: [_; PARTIES] = core::array::from_fn(|dealer_id| {
        let y_0 = F::random(&mut rng);
        deal::<F, G, THRESHOLD>(
            &mut rng,
            THRESHOLD,
            &y_0,
            &shamir_xs[..],
            &mut shamir_ys[dealer_id][..],
        )
        .expect("deal")
    });

    transpose(&mut shamir_ys);

    let results: [_; PARTIES] = core::array::from_fn(|i| {
        let shamir_x = &shamir_xs[i];
        let shamir_ys = &shamir_ys[i][..];

        let mut complaints = [false; PARTIES];
        aggregate::<F, G>(&vss_commitments[..], shamir_x, shamir_ys, &mut complaints[..])
            .expect("aggregate")
    });

    let shamir_ys = results.map(|(y, _)| y);
    let public_key = results
        .into_iter()
        .map(|(_, y)| y)
        .reduce(|left, right| {
            assert_eq!(left, right);
            left
        })
        .expect("zero participants?");

    for i in 0..PARTIES {
        let xs: [_; THRESHOLD] = core::array::from_fn(|j| shamir_xs[(i + j) % PARTIES]);
        let ys: [_; THRESHOLD] = core::array::from_fn(|j| shamir_ys[(i + j) % PARTIES]);

        let mut y = F::ZERO;
        for j in 0..THRESHOLD {
            y += ys[j] * xs.lagrange_coefficient_at(j, F::ZERO);
        }

        assert_eq!(g * y, public_key);
    }
}

fn transpose<T, const N: usize>(m: &mut [[T; N]; N]) {
    for i in 0..N {
        for j in 0..N {
            if i < j {
                let (has_i_row_at_i, has_j_row_at_0) = m.split_at_mut(j);
                let i_row = &mut has_i_row_at_i[i];
                let j_row = &mut has_j_row_at_0[0];

                let i_j_cell = &mut i_row[j];
                let j_i_cell = &mut j_row[i];

                core::mem::swap(i_j_cell, j_i_cell);
            }
        }
    }
}

#[test]
fn basic() {
    basic_impl::<Scalar, Point, 20, 5>(&mut rand::rngs::OsRng);
}
