use crate::config::DevupConfig;
use crate::errors::*;
use crate::uri::URI;
use std::path::PathBuf;
use std::process::Command;
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
    let user = match &self.username {
      Some(user) => user.to_owned(),
      None => conf.get_remote_username()?,
    };

    let conf_source = conf.get_remote_host();
    let source = match &self.devbox {
      Some(uri) => uri.to_string(),
      None => URI::from_str(&conf_source?)?.to_string(),
    };

    let conf_dest = PathBuf::from(conf.get_config_path()?);
    let dest = match &self.destination {
      Some(dest) => dest,
      None => &conf_dest,
    }
    .to_str()
    .ok_or("Failed to build local path")?;

    let output = Command::new("scp")
      .arg(format!("{}@{}:{}", user, source, dest))
      .arg(&dest)
      .output()?;

    if !output.status.success() {
      Err("Failed to transfer ini files!")?;
    }
    Ok(())
  }
}
