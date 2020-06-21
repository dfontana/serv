#![recursion_limit = "1024"]

extern crate structopt;
#[macro_use]
extern crate error_chain;
extern crate confy;
extern crate git2;
extern crate serde;

mod branch;
mod cacerts;
mod config;
mod errors;
mod inis;
mod uri;

use branch::List;
use cacerts::Certs;
use config::DevupConfig;
use inis::Inis;
use structopt::StructOpt;

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
  let args = Cli::from_args();
  let conf: DevupConfig = config::load("devup").unwrap();
  let res = match args {
    Cli::Inis(inis) => inis.run(conf),
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
