use cli_alice::cli::Cli;
use cli_alice::AnyError;

fn main() -> Result<(), AnyError> {
    let cli = Cli::bootstrap(std::env::args());

    cli.run(())
}
