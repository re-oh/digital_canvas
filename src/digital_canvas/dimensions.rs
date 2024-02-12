
pub struct Dimensions {
  pub width: u32,
  pub height: u32,
}

impl Dimensions {
  fn new(width: u32, height: u32) -> Self {
    Self {
      width,
      height
    }
  }

  fn to_tuple(&self) -> (u32, u32) {
    (self.width, self.height)
  }

}