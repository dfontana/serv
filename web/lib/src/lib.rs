extern crate image;
extern crate imageproc;
extern crate rand;
extern crate regex;
#[macro_use]
extern crate lazy_static;

mod block;
mod built_image;
mod encode;
mod fractal_tree;
mod render;

pub use built_image::run_test;
pub use fractal_tree::build_image;
