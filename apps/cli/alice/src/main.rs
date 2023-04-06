type AnyError = Box<dyn std::error::Error + Send + Sync + 'static>;

mod cli;
mod curve;
mod logging;
mod namespace;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), AnyError> {
    <cli::Cli as structopt::StructOpt>::from_args().run().await
}
