#![no_std]

mod dkls_tss;
pub use dkls_tss::{a, b};

#[cfg(test)]
mod demo;

#[cfg(test)]
mod tests;

#[cfg(test)]
#[macro_use]
extern crate std;
