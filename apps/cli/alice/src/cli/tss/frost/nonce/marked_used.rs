use std::io::BufRead;

use structopt::StructOpt;

use crate::AnyError;

use super::{Cli, Frost, Nonce, Tss};

#[derive(Debug, StructOpt)]
pub struct MarkUsed {
    #[structopt(name = "COMMITMENT")]
    commitment: Option<String>,
}

#[derive(Debug)]
enum Select {
    One(String),
    Stdin,
}

impl MarkUsed {
    pub async fn run(
        &self,
        _nonce: &Nonce,
        frost: &Frost,
        _tss: &Tss,
        cli: &Cli,
    ) -> Result<(), AnyError> {
        let key_prefix_ready = cli.secrets_ns.tss_frost_nonce_ready(frost.curve);
        let key_prefix_used = cli.secrets_ns.tss_frost_nonce_used(frost.curve);

        let selection = if let Some(one) = self.commitment.as_ref() {
            vec![one.to_owned()]
        } else {
            std::io::stdin().lock().lines().collect::<Result<Vec<_>, _>>()?
        };

        cli.with_secrets_manager(|mut sm| async move {
            for commitment in selection {
                let key_ready = format!("{}/{}", key_prefix_ready, commitment);
                let key_used = format!("{}/{}", key_prefix_used, commitment);
                tracing::debug!("removing key {:?}", key_ready);
                if let Err(reason) = sm.remove(&key_ready) {
                    tracing::warn!("could not remove {:?}: {}", commitment, reason);
                } else {
                    sm.set(&key_used, String::new());
                }
            }
            sm.save()?;
            Ok::<_, AnyError>(())
        })
        .await??;

        Ok(())
    }
}

impl std::str::FromStr for Select {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let out = if s == "--" { Self::Stdin } else { Self::One(s.to_owned()) };

        Ok(out)
    }
}
