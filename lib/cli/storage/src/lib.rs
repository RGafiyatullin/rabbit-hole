pub type AnyError = Box<dyn std::error::Error + Send + Sync + 'static>;

mod storage;
pub use storage::Storage;

mod table;
pub use table::Table;
