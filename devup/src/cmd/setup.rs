use crate::config::{DevupConfig, FILE_NAME};
use crate::errors::*;
use confy;
use dialoguer::{theme::ColorfulTheme, Confirm, Input};
use std::path::Path;
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Setup {}

impl Setup {
  pub fn run(&self) -> Result<()> {
    let theme = ColorfulTheme::default();
    println!("Welcome to Setup");

    let local_config_path = Input::with_theme(&theme)
      .with_prompt("Absolute Path to Local Configs")
      .validate_with(|input: &str| -> Result<()> {
        match Path::new(input).exists() {
          true => Ok(()),
          false => Err("Path does not exist")?,
        }
      })
      .interact()?;
    let remote_username = Input::with_theme(&theme)
      .with_prompt("Remote Host's Username (blank = $USER)")
      .allow_empty(true)
      .interact()?;
    let remote_host = Input::with_theme(&theme)
      .with_prompt("Remote Host's Location (blank = $DEVBOX)")
      .allow_empty(true)
      .interact()?;

    if !Confirm::with_theme(&theme)
      .with_prompt("Save?")
      .interact()?
    {
      return Ok(());
    }

    let config = DevupConfig {
      local_config_path: Some(local_config_path),
      remote_host: Some(remote_host),
      remote_username: Some(remote_username),
    };
    confy::store(FILE_NAME, config).chain_err(|| "Failed to save config file")
  }
}
