use cli_alice::caps::io::ProcessIO;
use cli_alice::{cli, AnyError};

fn main() -> Result<(), AnyError> {
    let cli = cli::Cli::create(std::env::args());
    let ret_code = cli::run(&cli, rand::rngs::OsRng, ProcessIO)?;

    std::process::exit(ret_code)
}
