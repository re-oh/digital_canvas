pub struct Cords2D {
  pub x: u32,
  pub y: u32,
}

impl Cords2D {
  pub fn new( x: u32, y: u32 ) -> Self {
    Self {x, y}
  }

  pub fn to_tuple ( &self ) -> ( u32, u32 ) {
    ( self.x, self.y )
  }

  pub fn distance(&self, cords: &Cords2D) -> u32{
    ((((( self.x as i32 - cords.x as i32  ) as f32).powi(2)) + ((( self.y as i32 - cords.y as i32 ) as f32).powi(2))).sqrt().round()) as u32
  }

  pub fn centroid(&self, cords: &Cords2D) -> Cords2D {
    Cords2D::new((self.x + cords.x) /  2, (self.y + cords.y) / 2)
  }

}