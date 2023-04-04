type Scalar = curve25519_dalek::scalar::Scalar;
type Point = curve25519_dalek::edwards::EdwardsPoint;

#[path = "generic/schnorr_proof.rs"]
mod schnorr_proof;

#[path = "generic/shamir_sss.rs"]
mod shamir_sss;

#[path = "generic/feldman_vsss.rs"]
mod feldman_vsss;

#[path = "generic/csi_rashi_dkg.rs"]
mod csi_rashi_dkg;
