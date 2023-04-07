use std::ffi::OsString;

use digest::Digest;
use ff::PrimeField;
use group::Group;
use structopt::StructOpt;

use crate::common::{Curve, HashFunction};

use super::{Cli, CliRun};

impl Cli<(), (), ()> {
    pub fn bootstrap(
        args: impl IntoIterator<Item = impl Into<OsString> + Clone>,
    ) -> Box<dyn CliRun<()>> {
        let args = args.into_iter().map(Into::into).collect::<Vec<_>>();
        <Self as StructOpt>::from_iter(args.iter().cloned()).run_0(args)
    }

    fn run_0(
        self,
        args: impl IntoIterator<Item = impl Into<OsString> + Clone>,
    ) -> Box<dyn CliRun<()>> {
        match self.curve {
            Curve::Secp256k1 => self.run_1::<k256::Scalar, k256::ProjectivePoint>(args),
            Curve::Ed25519 =>
                self.run_1::<curve25519::scalar::Scalar, curve25519::edwards::EdwardsPoint>(args),
            Curve::Ristretto25519 => self
                .run_1::<curve25519::scalar::Scalar, curve25519::ristretto::RistrettoPoint>(args),
        }
    }
    fn run_1<F, G>(
        self,
        args: impl IntoIterator<Item = impl Into<OsString> + Clone>,
    ) -> Box<dyn CliRun<()>>
    where
        F: PrimeField,
        G: Group<Scalar = F>,
    {
        match self.hash_function {
            HashFunction::Sha3_256 => self.run_2::<F, G, sha3::Sha3_256>(args),
        }
    }
    fn run_2<F, G, H>(
        self,
        args: impl IntoIterator<Item = impl Into<OsString> + Clone>,
    ) -> Box<dyn CliRun<()>>
    where
        F: PrimeField + 'static,
        G: Group<Scalar = F> + 'static,
        H: Digest + 'static,
    {
        Box::new(<Cli<F, G, H> as StructOpt>::from_iter(args))
    }
}
