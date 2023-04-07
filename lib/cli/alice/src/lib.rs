pub type AnyError = Box<dyn std::error::Error + Send + Sync + 'static>;

pub mod cli;
pub mod common;
