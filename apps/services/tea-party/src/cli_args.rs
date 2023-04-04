use std::net::SocketAddr;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct CliArgs {
    #[structopt(long, env = "BIND_ADDR")]
    pub bind_addr: SocketAddr,
}
