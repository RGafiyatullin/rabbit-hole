use digest::Digest;
use ff::{Field, PrimeField};
use group::{Group, Curve, GroupEncoding};
use k256::elliptic_curve::point::AffineCoordinates;
use rand::RngCore;


fn playground<F, G, H, const PARTIES: usize>(sk_a: F, sk_b: F, m: F) where F: PrimeField, G: Group<Scalar = F> + GroupEncoding + Curve, G::AffineRepr: AffineCoordinates<FieldRepr = F::Repr>, H: Digest  {
    let mut rng = rand::rngs::OsRng;

    let g = G::generator();
    let pk = g * (sk_a + sk_b);
    
    // Bob
    let k_b = F::random(&mut rng);
    let k_b_inv = k_b.invert().unwrap();
    let d_b = g * k_b;

    // Alice
    let phi = F::random(&mut rng);
    let k_a_seed = F::random(&mut rng);
    let r_ = d_b * k_a_seed;
    let k_a: F = utils::bytes_to_scalar(H::new().chain_update(r_.to_bytes()).finalize().as_ref());
    let k_a_inv = k_a.invert().unwrap();
    
    let r = d_b * k_a;
    let r_x = F::from_repr(r.to_affine().x()).unwrap();

    let (t_1_a, t_1_b) = mul(&mut rng, phi + k_a.invert().unwrap(), k_b.invert().unwrap());
    assert_eq!(t_1_a + t_1_b, (phi + k_a_inv) * k_b_inv);

    let (t_2a_a, t_2a_b) = mul(&mut rng, sk_a * k_a_inv, k_b_inv);
    assert_eq!(t_2a_a + t_2a_b, sk_a * k_a_inv * k_b_inv);

    let (t_2b_a, t_2b_b) = mul(&mut rng, k_a_inv, sk_b * k_b_inv);
    assert_eq!(t_2b_a + t_2b_b, k_a_inv * sk_b * k_b_inv);


    let t_2_a = t_2a_a + t_2b_a;
    let t_2_b = t_2a_b + t_2b_b;

    let sig_a = m * t_1_a + r_x * t_2_a;

    let theta = t_1_b - phi * k_b_inv;
    let sig_b = m * theta + r_x * t_2_b;

    let sig = sig_a + sig_b;

    let r_expected = (g * m + pk * r_x) * sig.invert().unwrap();
    let r_x_expected = F::from_repr(r_expected.to_affine().x()).unwrap();

    assert_eq!(r_x_expected, r_x);
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

