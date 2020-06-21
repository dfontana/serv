mod errors {
  use confy;
  use git2;
  use std::io;
  use std::str;
  error_chain! {
    foreign_links {
      Io(io::Error);
      Utf8(str::Utf8Error);
      Git2(git2::Error);
      Confy(confy::ConfyError);
    }
  }
}

pub use errors::*;
