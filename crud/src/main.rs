#[macro_use]
extern crate derive_builder;

use actix_web::{
  http,
  middleware::{errhandlers::ErrorHandlers, DefaultHeaders, Logger},
  App, HttpServer,
};
use env_logger::Env;
use listenfd::ListenFd;
use mongodb::{options::ClientOptions, Client};

use errors::{handle_404, handle_500};
use transaction::config;

mod errors;
mod transaction;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
  let mut listenfd = ListenFd::from_env();
  env_logger::from_env(Env::default().default_filter_or("info")).init();

  let mut client_options = ClientOptions::parse("mongodb://localhost:27017")
    .await
    .unwrap();
  client_options.app_name = Some("My App".to_string());
  let client = Client::with_options(client_options).unwrap();

  let mut server = HttpServer::new(move || {
    App::new()
      .configure(|cfg| config(cfg, client.clone()))
      .wrap(
        ErrorHandlers::new()
          .handler(http::StatusCode::INTERNAL_SERVER_ERROR, handle_500)
          .handler(http::StatusCode::NOT_FOUND, handle_404),
      )
      .wrap(Logger::new("'%r' %s %D"))
      .wrap(DefaultHeaders::new().header("Content-Type", "application/json"))
  });

  server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
    server.listen(l)?
  } else {
    server.bind("127.0.0.1:3000")?
  };

  server.run().await
}
