#![recursion_limit = "1024"]

extern crate structopt;
#[macro_use]
extern crate error_chain;

mod cacerts;
mod errors;
mod inis;
mod uri;

use cacerts::set_certs;
use inis::Pull;
use structopt::StructOpt;
use uri::URI;

use std::path::PathBuf;

#[derive(StructOpt)]
struct Cli {
  username: String,
  devbox: URI,
  #[structopt(parse(from_os_str))]
  destination: Option<PathBuf>,
}

fn main() {
  // TODO try to default args when missing from config
  let args = Cli::from_args();

  // First set the certs on the local machine
  match set_certs() {
    Err(e) => {
      eprintln!("Error: {}", e);
      std::process::exit(1);
    }
    _ => (),
  }

  // Next copy the inis
  let success = Pull::new()
    .user(args.username)
    .source(args.devbox)
    .dest(args.destination.or(Some(PathBuf::from("/test"))).unwrap())
    .run();
  match success {
    Err(e) => {
      eprintln!("Error: {}", e);
      std::process::exit(1);
    }
    _ => (),
  }
}
