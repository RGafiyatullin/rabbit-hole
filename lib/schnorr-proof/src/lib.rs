#![no_std]

mod schnorr_proof;
pub use crate::schnorr_proof::{prove, verify};
