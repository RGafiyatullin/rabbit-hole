use std::path::PathBuf;
use std::sync::Arc;

use cli_storage::{Storage, StorageOpen};
use lockfile::Lockfile;

#[derive(Debug, Clone)]
pub struct StorageSled {
    sled_db: sled::Db,

    // mind the drop order: keep it the last element in the struct
    _lockfile: Arc<Lockfile>,
}

impl Storage for StorageSled {
    fn flush(&self) -> Result<(), cli_storage::AnyError> {
        tracing::debug!("about to flush");
        let bytes_flushed = self.sled_db.flush()?;
        tracing::debug!("flushed {} bytes", bytes_flushed);
        Ok(())
    }
}

impl StorageOpen for StorageSled {
    fn open(arg: &str) -> Result<Self, cli_storage::AnyError> {
        let path: PathBuf = arg.parse()?;
        let sled_path = path.join("storage-sled.db");
        let lock_path = path.join("storage-sled.lock");

        let lockfile = Lockfile::create_with_parents(lock_path)?;

        let sled_db = sled::open(sled_path)?;

        Ok(Self { sled_db, _lockfile: Arc::new(lockfile) })
    }
}
