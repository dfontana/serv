#![recursion_limit = "1024"]

extern crate structopt;
#[macro_use]
extern crate error_chain;
extern crate confy;
extern crate git2;
extern crate serde;

mod cmd;
mod config;
mod errors;
mod uri;

use cmd::{Certs, Inis, List, Setup};
use config::DevupConfig;
use structopt::StructOpt;

#[derive(StructOpt)]
enum Cli {
  #[structopt(about = "Clone configurations to your machine")]
  Inis(Inis),

  #[structopt(about = "Configure SSL Certs for local Java")]
  Certs(Certs),

  #[structopt(about = "List branches you own for the current repo")]
  List(List),

  #[structopt(about = "Configure the DevUp tool")]
  Setup(Setup),
}

fn main() {
  let args = Cli::from_args();
  let conf: DevupConfig = config::load("devup").unwrap();
  let res = match args {
    Cli::Setup(setup) => setup.run(),
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
