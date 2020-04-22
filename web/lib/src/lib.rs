extern crate image;
extern crate imageproc;
extern crate rand;
extern crate regex;
#[macro_use]
extern crate lazy_static;

mod block;
mod render;
mod encode;
mod built_image;
mod fractal_tree;

pub use fractal_tree::build_image;
pub use built_image::run_test;
