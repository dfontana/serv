#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate lib;

use lib::creative::{Creative, Size};
use rocket::Request;
use rocket_contrib::json::JsonValue;

mod routes;

#[catch(404)]
fn not_found(req: &Request) -> JsonValue {
  json!({
    "error":
      format!(
        "Unknown path: {}, {}",
        req.uri(),
        Creative::new(Size::new(32, 32)).area()
      )
  })
}

fn main() {
  rocket::ignite()
    .mount("/image", routes![routes::get_image])
    .register(catchers![not_found])
    .launch();
}
