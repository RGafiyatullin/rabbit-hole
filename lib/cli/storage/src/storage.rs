use std::path::PathBuf;
use std::sync::Arc;

use lockfile::Lockfile;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::{AnyError, Table};

#[derive(Debug, Clone)]
pub struct Storage {
    pub(crate) sled_db: sled::Db,

    // mind the drop order: keep it the last element in the struct
    lockfile: Arc<Lockfile>,
}

impl Drop for Storage {
    fn drop(&mut self) {
        // if this is the last clone of that storage
        if let Some(_) = Arc::get_mut(&mut self.lockfile) {
            self.sled_db.flush();
        }
    }
}

impl<'a> From<&'a Storage> for Storage {
    fn from(value: &'a Storage) -> Self {
        value.clone()
    }
}

impl Storage {
    pub fn open(arg: &str) -> Result<Self, AnyError> {
        let path: PathBuf = arg.parse()?;
        let sled_path = path.join("storage-sled.db");
        let lock_path = path.join("storage-sled.lock");

        let lockfile = Lockfile::create_with_parents(lock_path)?;

        let sled_db = sled::open(sled_path)?;

        Ok(Self { sled_db, lockfile: Arc::new(lockfile) })
    }

    pub fn flush(&self) -> Result<(), AnyError> {
        tracing::debug!("about to flush");
        let bytes_flushed = self.sled_db.flush()?;
        tracing::debug!("flushed {} bytes", bytes_flushed);
        Ok(())
    }

    pub(crate) fn serialize<S>(&self, item: &S) -> Result<Vec<u8>, AnyError>
    where
        S: Serialize,
    {
        let json = serde_json::to_vec(item)?;
        Ok(json)
    }

    pub(crate) fn deserialize<D>(&self, data: impl AsRef<[u8]>) -> Result<D, AnyError>
    where
        D: DeserializeOwned,
    {
        let item = serde_json::from_slice(data.as_ref())?;
        Ok(item)
    }
}
