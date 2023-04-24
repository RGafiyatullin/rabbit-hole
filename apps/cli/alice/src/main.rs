use cli_alice::caps::io::StdIO;
use cli_alice::cli::{Cli, CliRun};
use cli_alice::AnyError;

fn main() -> Result<(), AnyError> {
    let ret_code = Cli::create(std::env::args()).run((rand::rngs::OsRng, StdIO))?;
    std::process::exit(ret_code)
}
