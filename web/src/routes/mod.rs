use lib;

use std::io::Cursor;
use rocket::response::{Response};
use rocket::http::{Status, ContentType};

#[get("/")]
pub fn get_image() -> Response<'static> {
  let mut res = Response::build();
  let imgres = lib::build_image();
  let res = match imgres {
    Ok(img) => {
      res
        .header(ContentType::PNG)
        .status(Status::Ok)
        .chunked_body(Cursor::new(img), 1024)
    },
    Err(_) => {
      res
        .header(ContentType::Plain)
        .status(Status::InternalServerError)
        .sized_body(Cursor::new("Failed to build image"))
    }
  };
  res.finalize()
}
