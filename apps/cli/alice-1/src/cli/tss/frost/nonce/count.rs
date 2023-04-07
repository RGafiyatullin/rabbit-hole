use structopt::StructOpt;

use crate::AnyError;

use super::{Cli, Frost, Nonce, Tss};

#[derive(Debug, StructOpt)]
pub struct Count {}

impl Count {
    pub async fn run(
        &self,
        _nonce: &Nonce,
        frost: &Frost,
        _tss: &Tss,
        cli: &Cli,
    ) -> Result<(), AnyError> {
        let key_prefix = cli.secrets_ns.tss_frost_nonce_ready(frost.curve);
        let count = cli
            .with_secrets_manager(|sm| async move {
                sm.keys().filter(|k| k.starts_with(key_prefix.as_str())).count()
            })
            .await?;

        println!("count: {}", count);

        Ok(())
    }
}
