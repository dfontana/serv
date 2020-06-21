use crate::errors::*;
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Setup {}

impl Setup {
  pub fn run(&self) -> Result<()> {
    Err("Not impl'd")?
  }
}
