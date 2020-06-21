use super::config::DevupConfig;
use super::errors::*;
use super::uri::URI;
use std::path::PathBuf;
use std::str::FromStr;
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Inis {
  #[structopt(help = "Remote user to pull files as")]
  username: Option<String>,
  #[structopt(help = "Remote location to fetch from")]
  devbox: Option<URI>,
  #[structopt(parse(from_os_str), help = "Local path to place files")]
  destination: Option<PathBuf>,
}

impl Inis {
  pub fn run(&self, conf: DevupConfig) -> Result<()> {
    Pull::new()
      .user(&self.username.as_ref().unwrap_or(&conf.get_remote_username()?))
      .source(&self.devbox.as_ref().unwrap_or(&URI::from_str(&conf.get_remote_host()?)?))
      .dest(
        &self
          .destination
          .as_ref()
          .unwrap_or(&PathBuf::from(conf.get_config_path()?)),
      )
      .run()
  }
}

struct Pull {}

impl Pull {
  fn new() -> Pull {
    Pull {}
  }

  fn user(&self, user: &str) -> &Self {
    &self
  }

  fn source(&self, remote: &URI) -> &Self {
    &self
  }

  fn dest(&self, path: &PathBuf) -> &Self {
    &self
  }

  fn run(&self) -> Result<()> {
    Ok(Err("test")?)
  }
}
