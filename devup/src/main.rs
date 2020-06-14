#![recursion_limit = "1024"]

extern crate structopt;
#[macro_use]
extern crate error_chain;
extern crate confy;
extern crate serde;
extern crate git2;

mod branch;
mod cacerts;
mod errors;
mod inis;
mod uri;
mod config;

use branch::List;
use cacerts::Certs;
use inis::Inis;
use structopt::StructOpt;
use uri::URI;
use errors::*;
use config::{DevupConfig};

use std::path::PathBuf;
use std::str::FromStr;

#[derive(StructOpt)]
enum Cli {
  #[structopt(about = "Clone configurations to your machine")]
  Inis(Inis),

  #[structopt(about = "Configure SSL Certs for local Java")]
  Certs(Certs),

  #[structopt(about = "List branches you own for the current repo")]
  List(List),
}

fn main() {
  // TODO try to default args when missing from config
  let args = Cli::from_args();
  let conf = config::load("devup").unwrap();
  let res = match args {
    Cli::Inis(inis) => inis.run(),
    Cli::Certs(cert) => cert.run(),
    Cli::List(list) => list.run(),
  };
  match res {
    Err(e) => {
      eprintln!("Error: {}", e);
      std::process::exit(1);
    }
    _ => (),
  }
}
