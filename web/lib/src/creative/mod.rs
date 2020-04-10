pub mod size;

pub use size::Size;

pub struct Creative {
  size: Size,
}

impl Creative {
  pub fn new(size: Size) -> Creative {
    Creative { size: size }
  }

  pub fn area(&self) -> i32 {
    self.size.width() * self.size.height()
  }
}
