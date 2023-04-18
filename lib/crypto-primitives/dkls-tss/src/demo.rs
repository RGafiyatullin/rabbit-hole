use digest::Digest;
use elliptic_curve::point::AffineCoordinates;
use ff::{PrimeField};
use group::{Curve, Group, GroupEncoding};
use rand::RngCore;

fn playground<F, G, H>(sk_a: F, sk_b: F, m: F)
where
    F: PrimeField,
    G: Group<Scalar = F> + GroupEncoding + Curve,
    G::AffineRepr: AffineCoordinates<FieldRepr = F::Repr>,
    H: Digest,
{
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

    let (t_1_a, t_1_b) = mta(&mut rng, phi + k_a.invert().unwrap(), k_b.invert().unwrap());
    assert_eq!(t_1_a + t_1_b, (phi + k_a_inv) * k_b_inv);

    let (t_2a_a, t_2a_b) = mta(&mut rng, sk_a * k_a_inv, k_b_inv);
    assert_eq!(t_2a_a + t_2a_b, sk_a * k_a_inv * k_b_inv);

    let (t_2b_a, t_2b_b) = mta(&mut rng, k_a_inv, sk_b * k_b_inv);
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

fn demo<F, G, H>(alice_t_0_a: F, bob_t_0_b: F, m: F)
where
    F: PrimeField,
    G: Group<Scalar = F> + GroupEncoding + Curve,
    G::AffineRepr: AffineCoordinates<FieldRepr = F::Repr>,
    H: Digest,
{
    let mut rng = rand::rngs::OsRng;

    let g = G::generator();
    let pk = g * (alice_t_0_a + bob_t_0_b);

    // Bob chooses his secret key `k_b`
    let bob_k_b = F::random(&mut rng); // F::from(11);
    let bob_d_b = g * bob_k_b;

    // `d_b` is sent to Alice
    let alice_d_b = bob_d_b;

    // Alice chooses it's instance key `k_a` and computes `R`
    let alice_phi = F::random(&mut rng); //F::from(13);
    let alice_k_a_pr = F::random(&mut rng); // F::from(17);
    let alice_r_a_pr = alice_d_b * alice_k_a_pr;

    let alice_h = utils::bytes_to_scalar::<F>(H::digest(alice_r_a_pr.to_bytes()).as_ref());
    let alice_k_a = alice_h + alice_k_a_pr;
    let alice_r = alice_d_b * alice_k_a;
    let alice_r_x = F::from_repr(alice_r.to_affine().x()).unwrap();

    // MtA
    let (alice_t_1_a, bob_t_1_b) = mta(&mut rng, 
        (alice_phi + F::from(1)) * alice_k_a.invert().unwrap(), 
        bob_k_b.invert().unwrap(),
    );
    let (alice_t_2a_a, bob_t_2a_b) = mta(&mut rng, 
        bob_k_b.invert().unwrap(), 
        alice_t_0_a * alice_k_a.invert().unwrap(),
    );
    let (alice_t_2b_a, bob_t_2b_b) = mta(&mut rng, 
        alice_k_a.invert().unwrap(), 
        bob_t_0_b * bob_k_b.invert().unwrap(),
    );
    
    // Alice
    let alice_t2_a = alice_t_2a_a + alice_t_2b_a;

    // Bob
    let bob_t2_b = bob_t_2a_b + bob_t_2b_b;

    // `r_a_seed` is sent to Bob
    let bob_r_a_pr = alice_r_a_pr;

    // Bob computes `R`
    let bob_h = utils::bytes_to_scalar::<F>(H::digest(bob_r_a_pr.to_bytes()).as_ref());
    let bob_r = bob_r_a_pr + bob_d_b * bob_h;
    let bob_r_x = F::from_repr(bob_r.to_affine().x()).unwrap();

    let r_expected = g * (bob_k_b * (bob_h + alice_k_a_pr));
    assert_eq!(r_expected, bob_r);
    assert_eq!(r_expected, alice_r);

    // Alice
    // eprintln!("g: {:?}", g);
    // eprintln!("alice-phi:       {:?}", alice_phi);
    // eprintln!("alice-k-a:       {:?}", alice_k_a);
    // eprintln!("alice-r:         {:?}", alice_r);
    // eprintln!("alice-t-1a-a:    {:?}", alice_t_1_a);

    let alice_gamma_1 = g + g * alice_phi - alice_r * alice_t_1_a;
    // eprintln!("alice-gamma-1:   {:?}", alice_gamma_1);

    let alice_hg_1 = utils::bytes_to_scalar::<F>(H::digest(alice_gamma_1.to_bytes()).as_ref());
    let alice_eta_phi = alice_hg_1 + alice_phi;

    let alice_gamma_2 = pk * alice_t_1_a - g * alice_t_2a_a;
    let alice_hg_2 = utils::bytes_to_scalar::<F>(H::digest(alice_gamma_2.to_bytes()).as_ref());
    let alice_sig_a = m * alice_t_1_a + alice_r_x * alice_t_2a_a + alice_r_x * alice_t_2b_a;
    let alice_eta_sig = alice_sig_a + alice_hg_2;

    // Alice sends encrypted `phi` and `sig_a`
    let bob_eta_phi = alice_eta_phi;
    let bob_eta_sig = alice_eta_sig;

    // Bob
    let bob_gamma_1 = bob_r * bob_t_1_b;
    assert_eq!(alice_gamma_1, bob_gamma_1);
    let bob_hg_1 = utils::bytes_to_scalar::<F>(H::digest(bob_gamma_1.to_bytes()).as_ref());
    let bob_phi = bob_eta_phi - bob_hg_1;
    assert_eq!(alice_phi, bob_phi);

    let bob_theta = bob_t_1_b - bob_k_b * bob_phi;
    let bob_sig_b = m * bob_theta + bob_r_x * bob_t2_b;

    let bob_gamma_2 = g * bob_t2_b - pk * bob_theta;
    // assert_eq!(alice_gamma_2, bob_gamma_2);
    

}

fn mta<F: PrimeField>(rng: impl RngCore, alpha: F, beta: F) -> (F, F) {
    let s = alpha * beta;
    let x = F::random(rng);
    let y = s - x;

    assert_eq!(alpha * beta, x + y);

    (x, y)
}

#[test]
fn playground_secp256k1() {
    let m = k256::Scalar::from(7u64);
    let sk_a = k256::Scalar::from(3u64);
    let sk_b = k256::Scalar::from(5u64);

    playground::<k256::Scalar, k256::ProjectivePoint, sha3::Sha3_256>(sk_a, sk_b, m);
}

#[test]
fn demo_secp256k1() {
    let m = k256::Scalar::from(7u64);
    let sk_a = k256::Scalar::from(3u64);
    let sk_b = k256::Scalar::from(5u64);

    demo::<k256::Scalar, k256::ProjectivePoint, sha3::Sha3_256>(sk_a, sk_b, m);
}


#[test]
fn playground_curve_debug() {
    let m = curve_debug::FU32::from(7u64);
    let sk_a = curve_debug::FU32::from(3u64);
    let sk_b = curve_debug::FU32::from(5u64);

    playground::<curve_debug::FU32, curve_debug::GU32, sha3::Sha3_256>(sk_a, sk_b, m);
}

#[test]
fn demo_curve_debug() {
    let m = curve_debug::FU32::from(7u64);
    let sk_a = curve_debug::FU32::from(3u64);
    let sk_b = curve_debug::FU32::from(5u64);

    demo::<curve_debug::FU32, curve_debug::GU32, sha3::Sha3_256>(sk_a, sk_b, m);
}


