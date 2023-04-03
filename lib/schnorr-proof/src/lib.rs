#![no_std]

mod schnorr_proof;
pub use schnorr_proof::{prove, verify};
