use super::errors::*;
use super::URI;
use std::path::PathBuf;

pub struct Pull {}

impl Pull {
  pub fn new() -> Pull {
    Pull {}
  }

  pub fn user(&self, user: String) -> &Self {
    &self
  }

  pub fn source(&self, remote: URI) -> &Self {
    &self
  }

  pub fn dest(&self, path: PathBuf) -> &Self {
    &self
  }

  pub fn run(&self) -> Result<()> {
    Ok(Err("test")?)
  }
}
