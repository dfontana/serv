use lib;

use rocket::http::{ContentType, Status};
use rocket::response::Response;
use std::io::Cursor;

#[get("/")]
pub fn get_image() -> Response<'static> {
  let mut res = Response::build();
  let imgres = lib::build_image();
  let res = match imgres {
    Ok(img) => res
      .header(ContentType::PNG)
      .status(Status::Ok)
      .chunked_body(Cursor::new(img), 1024),
    Err(_) => res
      .header(ContentType::Plain)
      .status(Status::InternalServerError)
      .sized_body(Cursor::new("Failed to build image")),
  };
  res.finalize()
}

#[get("/test")]
pub fn test() -> String {
  lib::run_test()
}
