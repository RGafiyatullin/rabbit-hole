use std::io::Write;

use common_interop::curve_select::CurveSelect;
use common_interop::types::{Point, Scalar};
use ff::PrimeField;
use group::{Group, GroupEncoding};
use serde::{Deserialize, Serialize};
use structopt::StructOpt;

use cli_storage::Storage;

use crate::caps::IO;
use crate::cli::{CliRun, CliRunnable};
use crate::{AnyError, RetCode};

#[derive(Debug, StructOpt)]
pub struct CmdCsiRashi {
    #[structopt(long, short)]
    curve: CurveSelect,

    #[structopt(subcommand)]
    cmd: Cmd,
}

#[derive(Debug, StructOpt)]
enum Cmd {
    Reset(CmdReset),
    Deal(CmdDeal),
    Aggregate(CmdAggregate),
}

#[derive(Debug, StructOpt)]
struct CmdReset {}

#[derive(Debug, StructOpt)]
struct CmdDeal {}

#[derive(Debug, StructOpt)]
struct CmdAggregate {}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Session {
    s4_x: Scalar,
    s4_y: Scalar,
    commitment: Vec<Point>,
}

impl<I: IO> CliRun<(I, Storage)> for CmdCsiRashi {
    fn run(&self, (io, storage): (I, Storage)) -> Result<RetCode, AnyError> {
        match &self.cmd {
            Cmd::Reset(sub) => sub.run((io, storage, self)),
            Cmd::Deal(sub) => sub.run((io, storage, self)),
            Cmd::Aggregate(sub) => sub.run((io, storage, self)),
        }
    }
}

impl<I: IO> CliRun<(I, Storage, &CmdCsiRashi)> for CmdReset {
    fn run(
        &self,
        (io, storage, csi_rashi): (I, Storage, &CmdCsiRashi),
    ) -> Result<RetCode, AnyError> {
        fn run<F: PrimeField, G: Group<Scalar = F> + GroupEncoding>(
            io: impl IO,
            storage: Storage,
        ) -> Result<RetCode, AnyError> {
            writeln!(
                io.stderr(),
                "F: {}; G: {}",
                std::any::type_name::<F>(),
                std::any::type_name::<G>()
            )?;
            Ok(1)
        }

        let result_opt = specialize_call!(run, (io, storage), csi_rashi.curve, [
            (CurveSelect::Secp256k1 => k256::Scalar, k256::ProjectivePoint),
            (CurveSelect::Ed25519 => curve25519::scalar::Scalar, curve25519::edwards::EdwardsPoint),
            (CurveSelect::Ristretto25519 => curve25519::scalar::Scalar, curve25519::ristretto::RistrettoPoint),
        ]);
        result_opt.ok_or("Unsupported curve")?
    }
}

impl<I: IO> CliRun<(I, Storage, &CmdCsiRashi)> for CmdDeal {
    fn run(
        &self,
        (io, storage, csi_rashi): (I, Storage, &CmdCsiRashi),
    ) -> Result<RetCode, AnyError> {
        unimplemented!()
    }
}

impl<I: IO> CliRun<(I, Storage, &CmdCsiRashi)> for CmdAggregate {
    fn run(
        &self,
        (io, storage, csi_rashi): (I, Storage, &CmdCsiRashi),
    ) -> Result<RetCode, AnyError> {
        unimplemented!()
    }
}
