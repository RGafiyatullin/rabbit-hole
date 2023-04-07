use std::future::Future;
use std::path::PathBuf;
use std::str::FromStr;

use rand::RngCore;
use securestore::{KeySource, SecretsManager};

use crate::AnyError;

use super::Cli;

impl Cli {
    pub async fn with_secrets_manager<'a, F, Fut, Ret>(&self, f: F) -> Result<Ret, AnyError>
    where
        F: FnOnce(SecretsManager) -> Fut,
        Fut: Future<Output = Ret> + 'a,
    {
        let secrets_manager = self.load_secrets_manager()?;
        let ret = f(secrets_manager).await;
        Ok(ret)
    }

    fn load_secrets_manager(&self) -> Result<SecretsManager, AnyError> {
        let data_file = if let Some(data_file) = self.secrets_data_file.as_ref() {
            data_file.to_owned()
        } else if let Ok(home) = std::env::var("HOME") {
            let mut path = PathBuf::from_str(&home)?;
            path.push(".alice");
            path.push("sm-data.json");
            path
        } else {
            Err("no $HOME")?
        };

        let key_file = if let Some(key_file) = self.secrets_key_file.as_ref() {
            key_file.to_owned()
        } else if let Ok(home) = std::env::var("HOME") {
            let mut path = PathBuf::from_str(&home)?;
            path.push(".alice");
            path.push("sm-key.pem");
            path
        } else {
            Err("no $HOME")?
        };

        let sm = SecretsManager::load(data_file, KeySource::Path(&key_file))?;

        Ok(sm)
    }
}

impl Cli {
    pub fn rng(&self) -> impl RngCore {
        rand::rngs::OsRng
    }
}
