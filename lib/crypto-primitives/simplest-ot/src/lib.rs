#![no_std]

mod simplest_ot;
pub use simplest_ot::{receiver_choose, sender_init, sender_keys};

#[cfg(test)]
mod tests;

#[cfg(test)]
#[macro_use]
extern crate std;
