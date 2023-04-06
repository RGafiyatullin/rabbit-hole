#![no_std]

mod frost_tss;
pub use frost_tss::{aggregate, preprocess, sign, Error};

#[cfg(feature = "std-error")]
extern crate std;
