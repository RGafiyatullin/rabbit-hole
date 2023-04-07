use std::marker::PhantomData;
use std::path::PathBuf;
use std::str::FromStr;

use digest::Digest;
use ff::PrimeField;
use group::Group;
use structopt::StructOpt;

use crate::common::{Curve, HashFunction};
use crate::AnyError;

mod bootstrap;
mod storage;

mod cli_storage;

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
    cmd: Cmd,

    #[structopt(skip)]
    _pd: PhantomData<(F, G, H)>,
}

impl<F, G, H> CliRun<()> for Cli<F, G, H>
where
    F: PrimeField,
    G: Group<Scalar = F>,
    H: Digest,
{
    fn run(&self, (): ()) -> Result<(), AnyError> {
        eprintln!("F: {}", std::any::type_name::<F>());
        eprintln!("G: {}", std::any::type_name::<G>());
        eprintln!("H: {}", std::any::type_name::<H>());

        match &self.cmd {
            Cmd::Storage(sub) => sub.run(self),
            _ => Err("not implemented".into()),
        }
    }
}

#[derive(Debug, StructOpt)]
enum Cmd {
    Keys,
    Dkg,
    Tss,
    Storage(cli_storage::CliStorage),
}
