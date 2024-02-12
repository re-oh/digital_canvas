use image::{ImageBuffer, RgbaImage};
use super::dimensions::Dimensions;
use super::render::{GenericRenderData, Renderable};

pub struct Layer {
  image_buffer: RgbaImage,
  render_buffer: Vec<Box<dyn Renderable>>,
  dimensions: Dimensions,
  zindex: i32,
  layername: String
}

impl Layer {

  pub fn new( name: String, dimensions: Dimensions, zindex: i32 ) -> Self {
    Self {
      image_buffer: ImageBuffer::new(dimensions.width, dimensions.height),
      render_buffer: vec![],
      layername: name,
      dimensions,
      zindex
    }
  }

  pub fn set_zindex ( &mut self, z: i32 ) {
    self.zindex = z;
  }

  pub fn move_above ( &mut self, comp_layer:&Layer ) {
    if comp_layer.zindex < self.zindex {
      return;
    }

    self.zindex = comp_layer.zindex + 1;
  }

  pub fn move_below ( &mut self, comp_layer:&Layer ) {
    if comp_layer.zindex > self.zindex {
      return;
    }
    self.zindex = comp_layer.zindex - 1;
  }

  pub fn move_down (&mut self, layers: i32) {
    self.zindex += layers;
  }

  pub fn move_up (&mut self, layers: i32) {
    self.zindex -= layers;
  }
}