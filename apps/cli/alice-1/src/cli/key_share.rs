use structopt::StructOpt;

use crate::AnyError;

use super::Cli;

#[derive(Debug, StructOpt)]
pub struct KeyShare {
    #[structopt(subcommand)]
    cmd: Cmd,
}

#[derive(Debug, StructOpt)]
enum Cmd {
    List,
    Info {
        // #[structopt(long, short)]
        // key_id: String,
    },
    Remove {
        // #[structopt(long, short)]
        // key_id: String,
    },
}

impl KeyShare {
    pub async fn run(&self, cli: &Cli) -> Result<(), AnyError> {
        let key_prefix = cli.secrets_ns.key_share_s4();
        let key_prefix = key_prefix.as_str();

        match &self.cmd {
            Cmd::List => {
                cli.with_secrets_manager(|sm| async move {
                    for k in sm
                        .keys()
                        .filter(|k| k.starts_with(key_prefix))
                        .map(|k| k.trim_start_matches(key_prefix))
                    {
                        let Some(("", k)) = k.split_once('/') else {continue};
                        let Some((curve, key_id)) = k.split_once('/') else {continue};

                        println!("{}\t{}", curve, key_id);
                    }
                })
                .await?;
            },
            Cmd::Remove { .. } => Err("not implemented")?,
            Cmd::Info { .. } => Err("not implemented")?,
        }

        Ok(())
    }
}
