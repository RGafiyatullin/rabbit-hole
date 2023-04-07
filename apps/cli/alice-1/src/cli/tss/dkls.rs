use structopt::StructOpt;

use crate::AnyError;

use super::{Cli, Tss};

#[derive(Debug, StructOpt)]
pub struct Dkls {}

impl Dkls {
    pub async fn run(&self, _tss: &Tss, _cli: &Cli) -> Result<(), AnyError> {
        Err("unimplemented".into())
    }
}
