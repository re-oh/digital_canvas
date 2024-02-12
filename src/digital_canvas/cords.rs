pub struct Cords2D {
  pub x: u32,
  pub y: u32,
}

impl Cords2D {
  pub fn new(x: u32, y: u32) -> Self {
      Cords2D { x, y }
  }

  pub fn to_tuple(&self) -> (u32, u32) {
      (self.x, self.y)
  }

  pub fn distance(&self, other: &Cords2D) -> u32 {
      (((((self.x as i32 - other.x as i32) as f32).powi(2))
          + (((self.y as i32 - other.y as i32) as f32).powi(2)))
          .sqrt()
          .round()) as u32
  }

  pub fn centroid(&self, other: &Cords2D) -> Cords2D {
      Cords2D::new((self.x + other.x) / 2, (self.y + other.y) / 2)
  }

  pub fn delta(&self, other: &Cords2D) -> (i32, i32) {
    let dx = other.x as i32 - self.x as i32;
    let dy = other.y as i32 - self.y as i32;
    (dx, dy)
}
}