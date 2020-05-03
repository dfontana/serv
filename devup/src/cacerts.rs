use super::errors::*;
use std::path::PathBuf;
use std::process::Command;
use std::str;

pub fn set_certs() -> Result<()> {
  let path = get_java_home()?;
  let paths = vec![
    path.join("lib/security/cacerts"),
    path.join("jre/lib/security/cacerts"),
  ];

  let cacerts = paths
    .iter()
    .find(|p| p.exists())
    .expect("Failed to find a cacerts path");

  // TODO finish fetching certs & applying them.

  Ok(())
}

fn get_java_home() -> Result<PathBuf> {
  let output = Command::new("/usr/libexec/java_home")
    .arg("-v")
    .arg("1.8")
    .output()?;

  if !output.status.success() {
    Err("Failed to get Java Home")?;
  }

  Ok(PathBuf::from(str::from_utf8(&output.stdout)?.trim()))
}
