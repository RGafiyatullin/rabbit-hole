use digest::Digest;
use ff::{Field, PrimeField};
use group::{Group, Curve};
use k256::elliptic_curve::point::AffineCoordinates;
use rand::RngCore;


fn playground<F, G, H, const PARTIES: usize>(sk_a: F, sk_b: F, m: F) where F: PrimeField, G: Group<Scalar = F> + Curve, G::AffineRepr: AffineCoordinates, H: Digest  {
    let mut rng = rand::rngs::OsRng;

    let g = G::generator();
    let pk = g * (sk_a + sk_b);

    // Bob
    let k_b = F::random(&mut rng);
    let d_b = g * k_b;

    // Alice
    let k_a_seed = F::random(&mut rng);



}

fn mul<F: PrimeField>(mut rng: impl RngCore, alpha: F, beta: F) -> (F, F) {
    let s = alpha * beta;
    let x = F::random(rng);
    let y = s - x;

    (x, y)
}


#[test]
fn playground_secp256k1() {
    let m = k256::Scalar::from(5000u64);
    let sk_a = k256::Scalar::from(1000u64);
    let sk_b = k256::Scalar::from(2000u64);

    playground::<k256::Scalar, k256::ProjectivePoint, sha3::Sha3_256, 3>(sk_a, sk_b, m);
}

