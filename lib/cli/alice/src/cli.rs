use std::marker::PhantomData;

use structopt::StructOpt;

use crate::common::{Curve, HashFunction};
use crate::AnyError;

mod bootstrap;

mod cmd_storage;

#[derive(Debug, StructOpt)]
pub struct Cli<F, G, H> {
    #[structopt(long, short, env = "ALICE_CURVE", default_value = "secp256k1")]
    pub curve: Curve,

    #[structopt(long, short, env = "ALICE_HASH_FUNCTION", default_value = "sha3-256")]
    pub hash_function: HashFunction,

    #[structopt(subcommand)]
    cmd: Cmd,

    #[structopt(skip)]
    _pd: PhantomData<(F, G, H)>,
}

pub trait CliRun<Prev> {
    fn run(&self, prev: Prev) -> Result<(), AnyError>;
}

#[derive(Debug, StructOpt)]
enum Cmd {
    Keys,
    Dkg,
    Tss,
    Storage(cmd_storage::CmdStorage),
}
