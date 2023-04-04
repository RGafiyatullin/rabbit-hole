use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "Alice, a tool to attend Tea-Parties")]
pub struct CliArgs {
    #[structopt(subcommand)]
    pub cmd: Command,
}

#[derive(Debug, StructOpt)]
pub enum Command {
    TeaParty(TeaParty),
}

#[derive(Debug, StructOpt)]
pub struct TeaParty {
    /// path to ed25519.pem key-file
    #[structopt(short, long)]
    pub key: String,

    /// tea-party URL
    #[structopt()]
    pub url: url::Url,
}
