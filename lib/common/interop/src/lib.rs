pub type AnyError = Box<dyn std::error::Error + Send + Sync + 'static>;

pub mod curve_select;
pub mod hash_function_select;
pub mod s4_share;
pub mod transcript;
pub mod types;
