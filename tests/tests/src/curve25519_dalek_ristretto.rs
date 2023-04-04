type Scalar = curve25519::scalar::Scalar;
type Point = curve25519::ristretto::RistrettoPoint;

#[path = "generic/schnorr_proof.rs"]
mod schnorr_proof;

#[path = "generic/shamir_sss.rs"]
mod shamir_sss;

#[path = "generic/feldman_vsss.rs"]
mod feldman_vsss;

#[path = "generic/csi_rashi_dkg.rs"]
mod csi_rashi_dkg;
