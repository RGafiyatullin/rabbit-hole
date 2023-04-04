type AnyError = Box<dyn std::error::Error + Send + Sync + 'static>;

mod cli_args;
use cli_args::{CliArgs, Command};

mod commands;

#[tokio::main]
async fn main() -> Result<(), AnyError> {
    let cli_args: CliArgs = structopt::StructOpt::from_args();

    match &cli_args.cmd {
        Command::TeaParty(tea_party) => commands::tea_party(tea_party, &cli_args).await,
    }
}

/*

alice
    tea-party --key <pem.key> <url>

 */
