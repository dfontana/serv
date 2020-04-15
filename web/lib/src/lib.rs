extern crate image;
extern crate imageproc;

mod block;
mod render;

use image::{png::PNGEncoder, ImageBuffer, ImageError, Pixel, RgbImage};
use std::ops::Deref;

use block::{Block, BlockRenderer};
use render::{RenderTree, Visitable};

pub fn build_image() -> Result<Vec<u8>, ImageError> {
  // https://docs.rs/image/0.23.3/image/#fn.save_buffer.html
  // https://docs.rs/imageproc/0.20.0/imageproc/
  // https://rocket.rs/v0.4/guide/responses/#option
  // https://api.rocket.rs/v0.4/rocket/response/struct.ResponseBuilder.html
  // https://stackoverflow.com/questions/50731636/how-do-i-encode-a-rust-piston-image-and-get-the-result-in-memory
  let mut buf: RgbImage = ImageBuffer::new(200, 300);
  for (x, y, pixel) in buf.enumerate_pixels_mut() {
    let r = (0.3 * x as f32) as u8;
    let b = (0.3 * y as f32) as u8;
    *pixel = image::Rgb([r, 0, b]);
  }
  encode_image(&buf)
}

pub fn run_test() -> String {
  // TODO build a more complex tree.
  let tree = RenderTree::new(Block::Solid);
  let mut node_list = BlockRenderer::new();
  tree.accept(&mut node_list);
  format!("{:?}", node_list)
}

fn encode_image<P, Container>(img: &ImageBuffer<P, Container>) -> Result<Vec<u8>, ImageError>
where
  P: Pixel<Subpixel = u8> + 'static,
  Container: Deref<Target = [P::Subpixel]>,
{
  let mut out = Vec::new();
  let encoder = PNGEncoder::new(&mut out);
  encoder.encode(img, img.width(), img.height(), P::COLOR_TYPE)?;
  Ok(out)
}