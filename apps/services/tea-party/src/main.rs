type AnyError = Box<dyn std::error::Error + Send + Sync + 'static>;

mod cli_args;
use cli_args::CliArgs;

#[tokio::main]
async fn main() -> Result<(), AnyError> {
    let args: CliArgs = structopt::StructOpt::from_args();

    Ok(())
}
