
pub struct Dimensions {
  pub width: u32,
  pub height: u32,
}

impl Dimensions {
  pub fn new(width: u32, height: u32) -> Self {
    Self {
      width,
      height
    }
  }

  pub fn new_from_tuple(dims: (u32, u32)) -> Self {
    Self {
      width: dims.0,
      height: dims.1
    }
  }

  pub fn to_tuple(&self) -> (u32, u32) {
    (self.width, self.height)
  }

}