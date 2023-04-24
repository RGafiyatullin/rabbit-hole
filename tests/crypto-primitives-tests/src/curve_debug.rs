type Scalar = curve_debug::FU32;
type Point = curve_debug::GU32;

#[path = "generic/schnorr_proof.rs"]
mod schnorr_proof;

#[path = "generic/shamir_sss.rs"]
mod shamir_sss;

#[path = "generic/feldman_vsss.rs"]
mod feldman_vsss;

#[path = "generic/csi_rashi_dkg.rs"]
mod csi_rashi_dkg;

#[path = "generic/frost_tss.rs"]
mod frost_tss;

#[path = "generic/simplest_ot.rs"]
mod simplest_ot;

#[path = "generic/hmrt_mta.rs"]
mod hmrt_mta;

#[path = "generic/dkls.rs"]
mod dkls;
