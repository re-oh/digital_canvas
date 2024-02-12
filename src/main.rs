mod digital_canvas;

use digital_canvas::cords::Cords2D;

fn main( ) {
  let cords1 = Cords2D::new(56, 87);
  let cords2 = Cords2D::new(120, 682);

  let distance = cords1.distance(&cords2);

  println!("{}", distance)
}