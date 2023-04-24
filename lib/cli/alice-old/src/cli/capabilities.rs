use std::path::PathBuf;

use cli_storage::Storage;
use rand::RngCore;

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

    pub fn open_storage(&self) -> Result<Storage, AnyError> {
        if let Some(storage) = self.storage.borrow().as_ref() {
            return Ok(storage.clone())
        }

        let path = self.storage_path()?;
        let storage = Storage::open(path.to_str().ok_or("invalid path")?)?;

        *self.storage.borrow_mut() = Some(storage.clone());

        Ok(storage)
    }
}

impl<F, D, H> Cli<F, D, H> {
    pub fn rng(&self) -> impl RngCore {
        rand::rngs::OsRng
    }
}
