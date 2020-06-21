pub use confy::load;
use serde::{Deserialize, Serialize};

use super::errors::*;

pub const FILE_NAME: &str = "devup";

#[derive(Serialize, Deserialize)]
pub struct DevupConfig {
  pub local_config_path: Option<String>,
  pub remote_username: Option<String>,
  pub remote_host: Option<String>,
}

impl ::std::default::Default for DevupConfig {
  fn default() -> Self {
    Self {
      local_config_path: None,
      remote_username: std::env::var("USER").ok(),
      remote_host: std::env::var("DEVBOX").ok(),
    }
  }
}

impl DevupConfig {
  pub fn get_config_path(&self) -> Result<String> {
    match &self.local_config_path {
      Some(val) => Ok(val.to_owned()),
      None => Err("Did not set local config path in config")?,
    }
  }

  pub fn get_remote_username(&self) -> Result<String> {
    match &self.remote_username {
      Some(val) => Ok(val.to_owned()),
      None => Err("Did not set Remote Username in config")?,
    }
  }

  pub fn get_remote_host(&self) -> Result<String> {
    match &self.remote_host {
      Some(val) => Ok(val.to_owned()),
      None => Err("Did not set Remote Host in config")?,
    }
  }
}
