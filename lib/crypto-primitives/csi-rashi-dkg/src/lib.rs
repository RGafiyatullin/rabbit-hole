#![no_std]

mod csi_rashi_dkg;
pub use csi_rashi_dkg::{aggregate, deal, Error};

#[cfg(feature = "std-error")]
extern crate std;
