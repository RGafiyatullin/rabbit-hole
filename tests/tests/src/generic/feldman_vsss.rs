use ff::PrimeField;
use group::Group;
use rand::RngCore;

use ::feldman_vsss::{CommitmentInitFromScheme, CommitmentVerifyShare};
use ::shamir_sss::{SchemeInitFromSecret, SchemeIssueShare};

use super::*;

fn basic_impl<F, G>(mut rng: impl RngCore)
where
    F: PrimeField,
    G: Group<Scalar = F>,
{
    const PARTIES: usize = 10;
    const THRESHOLD: usize = 3;

    let xs: [_; PARTIES] = core::array::from_fn(|_| F::random(&mut rng));

    let ss: [_; PARTIES] = core::array::from_fn(|_| {
        let mut s: [F; THRESHOLD] = Default::default();
        s.init_from_secret(&F::random(&mut rng), &mut rng);
        s
    });
    let cs: [_; PARTIES] = ss.map(|s| {
        let mut c = [G::identity(); THRESHOLD];
        c.init_from_scheme(s.as_ref());
        c
    });

    let ys: [[_; PARTIES]; PARTIES] = core::array::from_fn(|dealer_idx| {
        core::array::from_fn(|recipient_idx| ss[dealer_idx].issue_share(xs[recipient_idx]))
    });

    for dealer_idx in 0..PARTIES {
        let c = &cs[dealer_idx];

        for recipient_idx in 0..PARTIES {
            let y = &ys[dealer_idx][recipient_idx];

            for r_i in 0..PARTIES {
                let x = &xs[r_i];
                let expecting_correct = r_i == recipient_idx;

                assert_eq!(expecting_correct, c.verify_share(&x, &y));
            }
        }
    }
}

#[test]
fn basic() {
    basic_impl::<Scalar, Point>(&mut rand::rngs::OsRng);
}
