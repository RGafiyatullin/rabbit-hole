use std::path::PathBuf;

use cli_storage::{BoxedStorage, StorageOpenBoxed};
use cli_storage_sled::StorageSled;

use crate::AnyError;

use super::Cli;

impl<F, D, H> Cli<F, D, H> {
    fn storage_path(&self) -> Result<PathBuf, AnyError> {
        if let Some(path) = self.storage_path.as_ref() {
            Ok(path.to_owned())
        } else if let Ok(path) = std::env::var("HOME") {
            let path = path.parse::<PathBuf>()?;
            Ok(path.join(".alice"))
        } else {
            Err("Failed to determine the storage-path".into())
        }
    }

    pub fn open_storage(&self) -> Result<BoxedStorage, AnyError> {
        let path = self.storage_path()?;
        eprintln!("initializing sled-storage at {:?}", path);
        let storage = StorageSled::open_boxed(path.to_str().ok_or("invalid path")?)?;
        Ok(storage)
    }
}
