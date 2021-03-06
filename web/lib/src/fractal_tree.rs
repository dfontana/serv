use super::encode::buf2png;
use image::{ImageBuffer, ImageError, Rgb, RgbImage};
use imageproc::drawing::{draw_convex_polygon_mut, Point};
use rand::Rng;

pub fn build_image() -> Result<Vec<u8>, ImageError> {
  let mut buf: RgbImage = ImageBuffer::new(1024, 768);
  for (x, y, pixel) in buf.enumerate_pixels_mut() {
    let r = (0.3 * x as f32) as u8;
    let b = (0.3 * y as f32) as u8;
    *pixel = Rgb([r, 0, b]);
  }
  let mut rng = rand::thread_rng();
  draw_tree(&mut buf, 512.0, 700.0, rng.gen_range(0.0, 20.0), 11);
  buf2png(&buf)
}

fn draw_tree(buf: &mut RgbImage, x1: f64, y1: f64, angle: f64, depth: i32) {
  let x2 = x1 as f64 + angle.to_radians().sin() * depth as f64 * 10.0;
  let y2 = y1 as f64 - angle.to_radians().cos() * depth as f64 * 10.0;
  let white = Rgb([255u8, 255u8, 255u8]);
  let thickness = 5.0 * depth as f64 * 0.2;

  let p1 = Point::new(x1.round() as i32, y1.round() as i32);
  let p2 = Point::new(((x1 + thickness).round()) as i32, y1.round() as i32);
  let p3 = Point::new(((x2 + thickness).round()) as i32, y2.round() as i32);
  let p4 = Point::new(x2.round() as i32, y2.round() as i32);

  if p1 == p4 {
    return;
  }

  draw_convex_polygon_mut(buf, &[p1, p2, p3, p4], white);

  if depth > 0 {
    draw_tree(buf, x2, y2, angle - 20.0, depth - 1);
    draw_tree(buf, x2, y2, angle + 20.0, depth - 1);
  }
}
