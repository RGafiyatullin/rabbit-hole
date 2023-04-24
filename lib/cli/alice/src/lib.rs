#[macro_use]
extern crate common_macros;

pub type RetCode = i32;
pub type AnyError = Box<dyn std::error::Error + Send + Sync + 'static>;

pub mod caps;
pub mod cli;
pub mod data;

#[cfg(test)]
pub mod tests;
