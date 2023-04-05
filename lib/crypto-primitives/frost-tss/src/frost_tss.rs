use digest::Digest;
use ff::PrimeField;
use group::{Group, GroupEncoding};
use rand::RngCore;

use shamir_sss::LagrangeCoefficientAt;

#[derive(Debug)]
pub enum Error {
    InvalidShard,
}

pub fn preprocess<F, G>(mut rng: impl RngCore, nonces: &mut [(F, F)], commitments: &mut [(G, G)])
where
    F: PrimeField,
    G: Group<Scalar = F>,
{
    assert_eq!(nonces.len(), commitments.len());

    let g = G::generator();
    for ((d, e), (cd, ce)) in nonces.iter_mut().zip(commitments.iter_mut()) {
        *d = F::random(&mut rng);
        *e = F::random(&mut rng);
        *cd = g * *d;
        *ce = g * *e;
    }
}

pub fn sign<F, G, H>(
    public_key: &G,
    participant_id: usize,
    shamir_y: &F,
    shamir_xs: &[F],
    nonce: &(F, F),
    commitments: &[(G, G)],
    produce_challenge: impl Fn(&G, &G) -> F,
) -> (G, G, F)
where
    F: PrimeField,
    G: Group<Scalar = F> + GroupEncoding,
    H: Digest,
{
    assert!(participant_id < shamir_xs.len());
    assert_eq!(shamir_xs.len(), commitments.len());

    let g = G::generator();

    let i = participant_id;
    let y = *public_key;

    let s = shamir_y;
    let y_i = g * s;

    let d = nonce.0;
    let e = nonce.1;
    let rho_i = rho::<F, G, H>(&shamir_xs[i], commitments);
    let k = d + e * rho_i;
    let r_i = g * k;

    let r = (0..shamir_xs.len())
        .map(|i| {
            let shamir_x = &shamir_xs[i];
            let rho = rho::<F, G, H>(shamir_x, commitments);
            let (cd, ce) = commitments[i];
            cd + ce * rho
        })
        .sum::<G>();

    let c = produce_challenge(&y, &r);

    let lambda = shamir_xs.lagrange_coefficient_at(i, F::ZERO);

    let z_i = k + lambda * s * c;

    (y_i, r_i, z_i)
}

pub fn aggregate<F, G, H>(
    shards: &[(G, G, F)],
    shamir_xs: &[F],
    commitments: &[(G, G)],
    complaints: &mut [bool],
    produce_challenge: impl Fn(&G, &G) -> F,
) -> Result<(G, G, F), Error>
where
    F: PrimeField,
    G: Group<Scalar = F> + GroupEncoding,
    H: Digest,
{
    assert_eq!(shards.len(), shamir_xs.len());
    assert_eq!(shards.len(), complaints.len());
    assert_eq!(shards.len(), commitments.len());

    let g = G::generator();

    let y = shards
        .iter()
        .enumerate()
        .map(|(i, &(y_i, _, _))| y_i * shamir_xs.lagrange_coefficient_at(i, F::ZERO))
        .sum::<G>();
    let r = shards.iter().map(|(_, r_i, _)| r_i).sum::<G>();
    let z = shards.iter().map(|(_, _, z_i)| z_i).sum::<F>();

    let c = produce_challenge(&y, &r);

    for i in 0..shamir_xs.len() {
        let (cd, ce) = commitments[i];
        let lambda_i = shamir_xs.lagrange_coefficient_at(i, F::ZERO);
        let rho_i = rho::<F, G, H>(&shamir_xs[i], commitments);

        let (y_i, r_i, z_i) = shards[i];

        let is_valid_r = r_i == cd + ce * rho_i;
        let is_valid_z = (g * z_i) == (r_i + y_i * (lambda_i * c));

        std::eprintln!("[{}] r-ok: {}; z-ok: {}", i, is_valid_r, is_valid_z);

        complaints[i] = !(is_valid_r && is_valid_z);
    }

    if complaints.iter().copied().any(core::convert::identity) {
        Err(Error::InvalidShard)
    } else {
        Ok((y, r, z))
    }
}

pub(crate) fn rho<F, G, H>(shamir_x: &F, commitments: &[(G, G)]) -> F
where
    F: PrimeField,
    G: Group<Scalar = F> + GroupEncoding,
    H: Digest,
{
    let mut hasher = H::new().chain_update(shamir_x.to_repr());
    for (cd, ce) in commitments {
        hasher.update(cd.to_bytes());
        hasher.update(ce.to_bytes());
    }
    utils::bytes_to_scalar(hasher.finalize().as_ref())
}
