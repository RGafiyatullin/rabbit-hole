use digest::Digest;
use elliptic_curve::point::AffineCoordinates;
use ff::PrimeField;
use group::{Curve, Group, GroupEncoding};

use dkls_tss::{a, b};

#[test]
fn basic_secp256k1() {
    basic::<k256::Scalar, k256::ProjectivePoint, sha3::Sha3_256>()
}

#[test]
fn basic_curve_debug() {
    basic::<curve_debug::FU32, curve_debug::GU32, sha3::Sha3_256>()
}

fn basic<F, G, H>()
where
    F: PrimeField,
    G: Group<Scalar = F> + GroupEncoding + Curve,
    G::AffineRepr: AffineCoordinates<FieldRepr = F::Repr>,
    H: Digest,
{
    const L: usize = 256;

    let mut rng = rand::rngs::OsRng;

    let g = G::generator();

    let alice_t_0_a = F::random(&mut rng);
    let bob_t_0_b = F::random(&mut rng);

    let pk = g * (alice_t_0_a + bob_t_0_b);

    let mut rng = rand::rngs::OsRng;

    let mut bob_k_b = F::ZERO;
    let mut b2a_d_b = G::identity();
    let mut bob_mta_a = [[F::ZERO; L]; 3];
    let mut bob_mta_d = [[F::ZERO; L]; 3];
    let mut b2a_mta_pa = [[G::identity(); L]; 3];

    b::presign_offer::<F, G, L>(
        &mut rng,
        &mut bob_k_b,
        &mut b2a_d_b,
        &mut bob_mta_a,
        &mut bob_mta_d,
        &mut b2a_mta_pa,
    );

    let mut alice_phi = F::ZERO;
    let mut alice_k_a = F::ZERO;
    let mut alice_r = G::identity();
    let mut a2b_r_seed = G::identity();
    let mut a2b_mta_pb = [[G::identity(); L]; 3];
    let mut alice_mta_k = [[G::identity(); L]; 3];
    let mut alice_mta_t = [[F::ZERO; L]; 3];
    let mut a2b_mta_s = [[F::ZERO; L]; 3];

    a::presign_choose::<F, G, H, L>(
        &mut rng,
        &alice_t_0_a,
        &b2a_d_b,
        &mut alice_phi,
        &mut alice_k_a,
        &mut alice_r,
        &mut a2b_r_seed,
        &b2a_mta_pa,
        &mut a2b_mta_pb,
        &mut alice_mta_k,
        &mut alice_mta_t,
        &mut a2b_mta_s,
    );

    let mut bob_t_1_b = F::ZERO;
    let mut bob_t_2_b = F::ZERO;
    let mut bob_r = G::identity();
    let mut bob_mta_d = [[F::ZERO; L]; 3];
    let mut b2a_mta_e = [[[F::ZERO; 2]; L]; 3];

    b::presign_reply::<F, G, H, L>(
        &a2b_r_seed,
        &bob_t_0_b,
        &bob_k_b,
        &b2a_d_b,
        &mut bob_t_1_b,
        &mut bob_t_2_b,
        &mut bob_r,
        &mut bob_mta_d,
        &bob_mta_a,
        &a2b_mta_pb,
        &a2b_mta_s,
        &mut b2a_mta_e,
    );

    let mut alice_t_1_a = F::ZERO;
    let mut alice_t_2_a = F::ZERO;
    a::presign_finalize::<F, G, H, L>(
        &mut alice_t_1_a,
        &mut alice_t_2_a,
        &a2b_mta_s,
        &b2a_mta_e,
        &alice_mta_t,
        &alice_mta_k,
    );

    let m = utils::bytes_to_scalar(H::digest("Hello there!").as_ref());

    let mut a2b_eta_phi = F::ZERO;
    let mut a2b_eta_sig_a = F::ZERO;
    a::sign::<F, G, H>(
        &pk,
        &alice_k_a,
        &alice_t_1_a,
        &alice_t_2_a,
        &alice_phi,
        &alice_r,
        &m,
        &mut a2b_eta_phi,
        &mut a2b_eta_sig_a,
    );

    let sig = b::sign::<F, G, H>(
        &pk,
        &a2b_eta_phi,
        &a2b_eta_sig_a,
        &bob_k_b,
        &bob_t_1_b,
        &bob_t_2_b,
        &bob_r,
        &m,
    );

    assert!(verify(sig, pk, F::from_repr(alice_r.to_affine().x()).unwrap(), m));
}

// fn batch<F, G, H>(batch_size: usize)
// where
//     F: PrimeField,
//     G: Group<Scalar = F> + GroupEncoding + Curve,
//     G::AffineRepr: AffineCoordinates<FieldRepr = F::Repr>,
//     H: Digest,
// {
//     const L: usize = 256;

//     let mut rng = rand::rngs::OsRng;

//     let g = G::generator();

//     let alice_t_0_a = F::random(&mut rng);
//     let bob_t_0_b = F::random(&mut rng);

//     let pk = g * (alice_t_0_a + bob_t_0_b);

//     let mut rng = rand::rngs::OsRng;

//     let mut alice_phi = F::ZERO;
//     let mut alice_k_a = F::ZERO;
//     let mut alice_r = G::identity();
//     let mut a2b_r_seed = G::identity();
//     let mut a2b_mta_pb = [[G::identity(); L]; 3];
//     let mut alice_mta_k = [[G::identity(); L]; 3];
//     let mut alice_mta_t = [[F::ZERO; L]; 3];
//     let mut a2b_mta_s = [[F::ZERO; L]; 3];

//     let mut alice_t_1_a = F::ZERO;
//     let mut alice_t_2_a = F::ZERO;

//     let mut bob_k_b = F::ZERO;
//     let mut b2a_d_b = G::identity();
//     let mut bob_mta_a = [[F::ZERO; L]; 3];
//     let mut bob_mta_d = [[F::ZERO; L]; 3];
//     let mut b2a_mta_pa = [[G::identity(); L]; 3];

//     let mut bob_t_1_b = F::ZERO;
//     let mut bob_t_2_b = F::ZERO;
//     let mut bob_r = G::identity();
//     let mut bob_mta_d = [[F::ZERO; L]; 3];
//     let mut b2a_mta_e = [[[F::ZERO; 2]; L]; 3];

//     b::presign_offer::<F, G, L>(
//         &mut rng,
//         &mut bob_k_b,
//         &mut b2a_d_b,
//         &mut bob_mta_a,
//         &mut bob_mta_d,
//         &mut b2a_mta_pa,
//     );

//     a::presign_choose::<F, G, H, L>(
//         &mut rng,
//         &alice_t_0_a,
//         &b2a_d_b,
//         &mut alice_phi,
//         &mut alice_k_a,
//         &mut alice_r,
//         &mut a2b_r_seed,
//         &b2a_mta_pa,
//         &mut a2b_mta_pb,
//         &mut alice_mta_k,
//         &mut alice_mta_t,
//         &mut a2b_mta_s,
//     );

//     b::presign_reply::<F, G, H, L>(
//         &a2b_r_seed,
//         &bob_t_0_b,
//         &bob_k_b,
//         &b2a_d_b,
//         &mut bob_t_1_b,
//         &mut bob_t_2_b,
//         &mut bob_r,
//         &mut bob_mta_d,
//         &bob_mta_a,
//         &a2b_mta_pb,
//         &a2b_mta_s,
//         &mut b2a_mta_e,
//     );

//     a::presign_finalize::<F, G, H, L>(
//         &mut alice_t_1_a,
//         &mut alice_t_2_a,
//         &a2b_mta_s,
//         &b2a_mta_e,
//         &alice_mta_t,
//         &alice_mta_k,
//     );

//     let m = utils::bytes_to_scalar(H::digest("Hello there!").as_ref());

//     let mut a2b_eta_phi = F::ZERO;
//     let mut a2b_eta_sig_a = F::ZERO;
//     a::sign::<F, G, H>(
//         &pk,
//         &alice_k_a,
//         &alice_t_1_a,
//         &alice_t_2_a,
//         &alice_phi,
//         &alice_r,
//         &m,
//         &mut a2b_eta_phi,
//         &mut a2b_eta_sig_a,
//     );

//     let sig = b::sign::<F, G, H>(
//         &pk,
//         &a2b_eta_phi,
//         &a2b_eta_sig_a,
//         &bob_k_b,
//         &bob_t_1_b,
//         &bob_t_2_b,
//         &bob_r,
//         &m,
//     );

//     assert!(verify(sig, pk, F::from_repr(alice_r.to_affine().x()).unwrap(), m));
// }

fn verify<F, G>(sig: F, pk: G, r_x_known: F, m: F) -> bool
where
    F: PrimeField,
    G: Group<Scalar = F> + GroupEncoding + Curve,
    G::AffineRepr: AffineCoordinates<FieldRepr = F::Repr>,
{
    let g = G::generator();
    let r_derived = (g * m + pk * r_x_known) * sig.invert().unwrap();
    let r_x_derived = F::from_repr(r_derived.to_affine().x()).unwrap();

    r_x_derived == r_x_known
}
