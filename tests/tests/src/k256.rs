type Scalar = k256::Scalar;
type Point = k256::ProjectivePoint;

#[path = "generic/schnorr_proof.rs"]
mod schnorr_proof;
#[path = "generic/shamir_sss.rs"]
mod shamir_sss;
