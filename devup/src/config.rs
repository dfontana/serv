pub use confy::{load};
use serde::{Serialize, Deserialize};

use super::errors::*;

#[derive(Serialize, Deserialize)]
pub struct DevupConfig {
    local_copy_path: String,
}

impl ::std::default::Default for DevupConfig {
    fn default() -> Self { Self { local_copy_path: "".into() } }
}

impl DevupConfig {
  pub fn get_copy_path(&self) -> Result<String> {
    if self.local_copy_path.is_empty() {
      Err("Must provide local copy path")?
    }
    Ok(self.local_copy_path.to_owned())
  }
}
