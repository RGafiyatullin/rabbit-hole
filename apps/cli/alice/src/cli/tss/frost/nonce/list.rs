use std::io::Write;

use structopt::StructOpt;

use crate::AnyError;

use super::{Cli, Frost, Nonce, Tss};

#[derive(Debug, StructOpt)]
pub struct List {}

impl List {
    pub async fn run(
        &self,
        _nonce: &Nonce,
        frost: &Frost,
        _tss: &Tss,
        cli: &Cli,
    ) -> Result<(), AnyError> {
        let key_prefix = cli.secrets_ns.tss_frost_nonce(frost.curve);
        let key_prefix = key_prefix.as_str();

        let mut stdout = std::io::stdout().lock();
        cli.with_secrets_manager(|sm| async move {
            for key in sm
                .keys()
                .filter(|k| k.starts_with(key_prefix))
                .map(|k| k.trim_start_matches(key_prefix))
            {
                let commitment = &key[key_prefix.as_bytes().len() + 1..];

                writeln!(&mut stdout, "{}", commitment)?;
            }

            Ok::<_, AnyError>(())
        })
        .await??;

        Ok(())
    }
}
