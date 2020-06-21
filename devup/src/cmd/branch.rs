use crate::errors::*;
use git2::{BranchType, Repository};
use std::env;
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct List {
  #[structopt(help = "Optional. Author's Email to filter branches by (otherwise taken from repo)")]
  filter: Option<String>,
}

impl List {
  pub fn run(self) -> Result<()> {
    let current_dir = env::current_dir().unwrap();
    let repo = Repository::discover(current_dir)?;
    let email = self
      .filter
      .or_else(|| {
        repo
          .signature()
          .ok()
          .and_then(|f| f.email().map(|k| k.to_string()))
      })
      .chain_err(|| "Unable to locate email in Git Config")?;
    let options = get_branches(&repo, &email)?;
    Ok(print_list(options))
  }
}

struct BranchOption {
  b_type: BranchType,
  b_name: String,
}

fn get_branches(repo: &Repository, email_filter: &str) -> Result<Vec<BranchOption>> {
  Ok(
    repo
      .branches(None)?
      .filter_map(|f| f.ok())
      .filter(|f| {
        f.0
          .get()
          .peel_to_commit()
          .map(|op| op.author().email().map(|e| e == email_filter).is_some())
          .is_ok()
      })
      .map(|(branch, b_type)| BranchOption {
        b_type,
        b_name: branch
          .into_reference()
          .shorthand()
          .get_or_insert("")
          .to_string(),
      })
      .collect(),
  )
}

fn print_list(list: Vec<BranchOption>) {
  list.iter().for_each(|f| {
    let location = match f.b_type {
      BranchType::Remote => "Remote",
      BranchType::Local => "Local",
    };
    println!("[{:<6}] {}", location, f.b_name)
  })
}
