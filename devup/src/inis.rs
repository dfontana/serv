use super::errors::*;
use super::uri::URI;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Inis {
  #[structopt(help = "Remote user to pull files as")]
  username: String,
  #[structopt(help = "Remote location to fetch from")]
  devbox: URI,
  #[structopt(parse(from_os_str), help = "Local path to place files")]
  destination: Option<PathBuf>,
}

impl Inis {
  pub fn run(&self) -> Result<()> {
    Pull::new()
      .user(&self.username)
      .source(&self.devbox)
      .dest(
        &self
          .destination
          .as_ref()
          .get_or_insert(&PathBuf::from("/test")),
      )
      .run()
  }

  fn copy_from_remote(args: Cli, conf: DevupConfig) -> Result<()> {
    let success = Pull::new()
      .user(args.username.unwrap_or(get_env("USER")?))
      .source(args.devbox.unwrap_or(URI::from_str(get_env("DEVBOX")?.as_str())?))
      .dest(args.destination.unwrap_or(PathBuf::from(conf.get_copy_path()?)))
      .run();
    Ok(())
  }

  fn get_env(var: &str) -> Result<String> {
    std::env::var("USER").chain_err(|| format!("Failed to get env ${}", var))
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
