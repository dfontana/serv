extern crate reqwest;
extern crate serde;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let client = reqwest::Client::new();

  let resp = client
    .get("https://httpbin.org/ip")
    .send()
    .await?
    .json::<HashMap<String, String>>()
    .await?;

  let resp2 = client
    .post("http::meow")
    .json(&Test { cats: 1, dogs: 2 })
    .send()
    .await?
    .json::<Test>()
    .await?;

  println!("{:#?}{:#?}", resp, resp2);

  Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
struct Test {
  cats: i32,
  dogs: i32,
}
