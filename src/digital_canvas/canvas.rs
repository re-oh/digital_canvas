use std::ops::RangeInclusive;
use image::{ImageBuffer, Rgba, RgbaImage};
use super::{dimensions::Dimensions, layer::Layer};
pub struct Canvas {
  layers:Vec<Layer>,
  dimensions:Dimensions,
  canvas_buffer: RgbaImage
}

impl Canvas {

  fn new(width:u32, height:u32) -> Self {
    Self {
      layers: vec![],
      dimensions: Dimensions::new(width, height),
      canvas_buffer: ImageBuffer::new(width, height)
    }
  }

  fn add_layer(&mut self, name: &str ) {
    self.layers.push(Layer::new(
      String::from(name), 
      Dimensions::new_from_tuple(self.dimensions.to_tuple()),
      self.layers.iter().max_by_key(|layer| layer.zindex).unwrap().zindex + 1,))
  }
}