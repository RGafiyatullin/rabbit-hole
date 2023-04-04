use feldman_vsss::{CommitmentInitFromScheme, CommitmentVerifyShare};
use ff::{Field, PrimeField};
use group::Group;
use rand::RngCore;

use shamir_sss::{SchemeInitFromSecret, SchemeIssueShare};

#[derive(Debug)]
#[cfg_attr(feature = "std-error", derive(thiserror::Error))]
pub enum Error {}

pub fn deal<F, G, const MAX_THRESHOLD: usize>(
    mut rng: impl RngCore,
    threshold: usize,
    secret: &F,
    shamir_xs: &[F],
    shamir_ys: &mut [F],
) -> Result<[G; MAX_THRESHOLD], Error>
where
    F: PrimeField,
    G: Group<Scalar = F>,
{
    assert_eq!(shamir_xs.len(), shamir_ys.len());
    assert!(threshold <= MAX_THRESHOLD);

    let mut ss = [F::ZERO; MAX_THRESHOLD];
    (&mut ss[0..threshold]).init_from_secret(secret, &mut rng);

    let mut sc = [G::identity(); MAX_THRESHOLD];
    sc.init_from_scheme(&ss);

    shamir_xs.iter().copied().zip(shamir_ys.iter_mut()).for_each(|(x, y)| {
        *y = ss.issue_share(x);
    });

    Ok(sc)
}

pub fn aggregate_deals<F, G>(
    vss_commitments: &[impl AsRef<[G]>],
    shamir_x: &F,
    shamir_ys: &[F],
    complaints: &mut [bool],
) -> Result<(F, G), Error>
where
    F: Field,
    G: Group<Scalar = F>,
{
    let parties_count = vss_commitments.len();
    assert_eq!(parties_count, shamir_ys.len());
    assert_eq!(parties_count, complaints.len());

    let key_share = complaints
        .iter_mut()
        .zip(vss_commitments)
        .zip(shamir_ys)
        .map(|((complaint, vss_commitment), shamir_y)| {
            (complaint, vss_commitment.as_ref(), shamir_y)
        })
        .map(|(complaint, vss_commitment, shamir_y)| {
            *complaint = !vss_commitment.verify_share(shamir_x, shamir_y);
            *shamir_y
        })
        .reduce(core::ops::Add::add)
        .expect("zero participants?");

    let public_key = vss_commitments
        .iter()
        .map(|c| c.as_ref()[0])
        .reduce(core::ops::Add::add)
        .expect("zero participants?");

    let out = (key_share, public_key);
    Ok(out)
}
