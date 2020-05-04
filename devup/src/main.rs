#![recursion_limit = "1024"]

extern crate structopt;
#[macro_use]
extern crate error_chain;
extern crate confy;
extern crate serde;

mod cacerts;
mod errors;
mod inis;
mod uri;
mod config;

use cacerts::set_certs;
use inis::Pull;
use structopt::StructOpt;
use uri::URI;
use errors::*;
use config::{DevupConfig};

use std::path::PathBuf;
use std::str::FromStr;

#[derive(StructOpt)]
struct Cli {
  username: Option<String>,
  devbox: Option<URI>,
  #[structopt(parse(from_os_str))]
  destination: Option<PathBuf>,
}

fn main() {
  // TODO try to default args when missing from config
  let args = Cli::from_args();
  let conf = config::load("devup").unwrap();

  // First set the certs on the local machine
  match set_certs() {
    Err(e) => {
      eprintln!("Error: {}", e);
      std::process::exit(1);
    }
    _ => (),
  }

  // Next copy the inis
  let success = copy_from_remote(args, conf);
  match success {
    Err(e) => {
      eprintln!("Error: {}", e);
      std::process::exit(1);
    }
    _ => (),
  }
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
