type Scalar = curve25519::scalar::Scalar;
type Point = curve25519::ristretto::RistrettoPoint;

#[path = "generic/schnorr_proof.rs"]
mod schnorr_proof;
#[path = "generic/shamir_sss.rs"]
mod shamir_sss;
