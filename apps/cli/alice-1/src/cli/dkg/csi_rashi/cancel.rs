use structopt::StructOpt;

use crate::AnyError;

use super::{Cli, CsiRashi, Dkg};

#[derive(Debug, StructOpt)]
pub struct Cancel {}

impl Cancel {
    pub async fn run(&self, csi_rashi: &CsiRashi, _dkg: &Dkg, cli: &Cli) -> Result<(), AnyError> {
        let key_id = csi_rashi.key_id.as_str();

        let state_key =
            format!("{}/{}", cli.secrets_ns.dkg_csi_rashi_own_share(csi_rashi.curve), key_id);

        cli.with_secrets_manager(|mut sm| async move {
            let _ = sm.remove(&state_key);
            sm.save()
        })
        .await??;

        Ok(())
    }
}
