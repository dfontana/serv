pub struct Size {
  width: i32,
  height: i32,
}

impl Size {
  pub fn new(w: i32, h: i32) -> Size {
    Size {
      width: w,
      height: h,
    }
  }

  pub fn width(&self) -> &i32 {
    &self.width
  }

  pub fn height(&self) -> &i32 {
    &self.height
  }
}
