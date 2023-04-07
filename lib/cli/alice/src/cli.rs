use std::marker::PhantomData;
use std::path::PathBuf;

use digest::Digest;
use ff::PrimeField;
use group::{Group, GroupEncoding};
use structopt::StructOpt;

use common_interop::types::{Curve, HashFunction};

use crate::AnyError;

mod bootstrap;
mod capabilities;

mod cli_dkg;
mod cli_storage;
mod cli_tss;

pub trait CliRun<Prev> {
    fn run(&self, prev: Prev) -> Result<(), AnyError>;
}

#[derive(Debug, StructOpt)]
pub struct Cli<F, G, H> {
    #[structopt(long, short, env = "ALICE_CURVE", default_value = "secp256k1")]
    pub curve: Curve,

    #[structopt(long, short, env = "ALICE_HASH_FUNCTION", default_value = "sha3-256")]
    pub hash_function: HashFunction,

    #[structopt(long, short, env = "ALICE_STORAGE_PATH")]
    pub storage_path: Option<PathBuf>,

    #[structopt(subcommand)]
    cmd: Cmd<F, G, H>,

    #[structopt(skip)]
    _pd: PhantomData<(F, G, H)>,
}

impl<F, G, H> CliRun<()> for Cli<F, G, H>
where
    F: PrimeField,
    G: Group<Scalar = F> + GroupEncoding,
    H: Digest,
{
    fn run(&self, (): ()) -> Result<(), AnyError> {
        // eprintln!("F: {}", std::any::type_name::<F>());
        // eprintln!("G: {}", std::any::type_name::<G>());
        // eprintln!("H: {}", std::any::type_name::<H>());
        match &self.cmd {
            Cmd::Storage(sub) => sub.run(self),
            Cmd::Dkg(sub) => sub.run(self),
            Cmd::Tss(sub) => sub.run(self),
            _ => Err("not implemented".into()),
        }
    }
}

#[derive(Debug, StructOpt)]
enum Cmd<F, G, H> {
    Keys,
    Dkg(cli_dkg::CliDkg<F, G, H>),
    Tss(cli_tss::CliTss<F, G, H>),
    Storage(cli_storage::CliStorage),
}
