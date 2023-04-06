use structopt::StructOpt;

use crate::AnyError;

use super::{Cli, Frost, Tss};

#[derive(Debug, StructOpt)]
pub struct Sign {}

impl Sign {
    pub async fn run(&self, _frost: &Frost, _tss: &Tss, _cli: &Cli) -> Result<(), AnyError> {
        Err("unimplemented".into())
    }
}
