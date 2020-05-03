mod errors {
  use std::io;
  use std::str;
  error_chain! {
    foreign_links {
      Io(io::Error);
      Utf8(str::Utf8Error);
    }
  }
}

pub use errors::*;
