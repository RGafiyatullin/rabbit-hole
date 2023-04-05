use digest::Digest;
use ff::PrimeField;
use group::{Group, GroupEncoding};
use rand::RngCore;

use ::shamir_sss::{SchemeInitFromSecret, SchemeIssueShare};

use ::frost_tss::*;

use super::*;

fn basic_impl<
    F,
    G,
    H,
    const PARTIES: usize,
    const THRESHOLD: usize,
    const PREPROCESS_COUNT: usize,
>(
    mut rng: impl RngCore,
) where
    F: PrimeField,
    G: Group<Scalar = F> + GroupEncoding,
    H: Digest,
{
    let g = G::generator();
    let secret_key = F::random(&mut rng);
    let public_key = g * secret_key;

    let ss = {
        let mut ss = [F::ZERO; THRESHOLD];
        ss.init_from_secret(&secret_key, &mut rng);
        ss
    };

    let shamir_xs: [_; PARTIES] = core::array::from_fn(|_| F::random(&mut rng));
    let shamir_ys: [_; PARTIES] = core::array::from_fn(|i| ss.issue_share(shamir_xs[i]));

    let mut nonces: [_; PARTIES] =
        core::array::from_fn(|_| vec![(F::ZERO, F::ZERO); PREPROCESS_COUNT]);
    let mut commitments: [_; PARTIES] =
        core::array::from_fn(|_| vec![(G::identity(), G::identity()); PREPROCESS_COUNT]);

    for i in 0..PARTIES {
        let nonces = &mut nonces[i][..];
        let commitments = &mut commitments[i][..];
        preprocess(&mut rng, nonces, commitments)
    }

    for i in 0.. {
        let message = format!("message #{}", i);

        let session_xs: [_; THRESHOLD] = core::array::from_fn(|j| shamir_xs[(i + j) % PARTIES]);
        let session_ys: [_; THRESHOLD] = core::array::from_fn(|j| shamir_ys[(i + j) % PARTIES]);
        let mut session_nonces = [(F::ZERO, F::ZERO); THRESHOLD];
        let mut session_commitments = [(G::identity(), G::identity()); THRESHOLD];

        for (j, slot) in session_nonces.iter_mut().enumerate() {
            let Some(nonce) = nonces[(i + j) % PARTIES].pop() else {return};
            *slot = nonce;
        }
        for (j, slot) in session_commitments.iter_mut().enumerate() {
            let Some(nonce) = commitments[(i + j) % PARTIES].pop() else {return};
            *slot = nonce;
        }

        let produce_challenge = |y: &G, r: &G| {
            ::utils::bytes_to_scalar(
                H::new()
                    .chain_update(y.to_bytes())
                    .chain_update(r.to_bytes())
                    .chain_update(message.as_bytes())
                    .finalize()
                    .as_ref(),
            )
        };

        let shards: [_; THRESHOLD] = core::array::from_fn(|j| {
            sign::<F, G, H>(
                &public_key,
                j,
                &session_ys[j],
                &session_xs,
                &session_nonces[j],
                &session_commitments,
                &produce_challenge,
            )
        });

        let mut complaints = [false; THRESHOLD];
        let (y, r, z) = aggregate::<F, G, H>(
            &shards,
            &session_xs,
            &session_commitments,
            &mut complaints,
            &produce_challenge,
        )
        .expect("aggregate");

        assert!(!complaints.into_iter().any(core::convert::identity));

        assert_eq!(g * z, r + y * produce_challenge(&y, &r));
    }
}

#[test]
fn basic() {
    basic_impl::<Scalar, Point, sha3::Sha3_256, 10, 3, 1_000>(&mut rand::rngs::OsRng);
}
