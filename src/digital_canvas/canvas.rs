use std::ops::RangeInclusive;
use image::{ImageBuffer, Rgba, RgbaImage};
use super::{dimensions::Dimensions, layer::Layer};
pub struct Canvas {
  layers:Vec<Layer>,
  dimensions:Dimensions,
  canvas_buffer: RgbaImage
}