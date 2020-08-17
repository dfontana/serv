#[macro_use]
extern crate derive_builder;

use actix_web::{
  http,
  middleware::{errhandlers::ErrorHandlers, DefaultHeaders, Logger},
  App, HttpServer,
};
use listenfd::ListenFd;
use mongodb::{options::ClientOptions, Client, Collection};

use errors::{handle_404, handle_500};
use transaction_service::TransactionService;

mod errors;
mod transaction_router;
mod transaction_service;

pub struct ServiceManager {
  txn: TransactionService,
}

impl ServiceManager {
  pub fn new(txn: TransactionService) -> Self {
    ServiceManager { txn }
  }
}

pub struct AppState {
  service_manager: ServiceManager,
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
  std::env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
  std::env::set_var("RUST_BACKTRACE", "1");
  env_logger::init();

  //  InitDB
  let client = init_client().await;
  let txn_coll = init_collections(client);

  // Init local listen for autoreload
  let listen = ListenFd::from_env().take_tcp_listener(0).unwrap();

  let mut server = HttpServer::new(move || {
    let txn = TransactionService::new(txn_coll.clone());
    let manager = ServiceManager::new(txn);

    App::new()
      .wrap(
        ErrorHandlers::new()
          .handler(http::StatusCode::INTERNAL_SERVER_ERROR, handle_500)
          .handler(http::StatusCode::NOT_FOUND, handle_404),
      )
      .wrap(Logger::new("'%r' %s %D"))
      .wrap(DefaultHeaders::new().header("Content-Type", "application/json"))
      .data(AppState {
        service_manager: manager,
      })
      .configure(transaction_router::init)
  });
  server = match listen {
    Some(l) => server.listen(l)?,
    None => server.bind("127.0.0.1:3000")?,
  };
  server.run().await
}

async fn init_client() -> Client {
  let db_url = "mongodb://localhost:27017";
  let client_options = ClientOptions::parse(db_url).await.unwrap();
  Client::with_options(client_options).unwrap()
}

fn init_collections(client: Client) -> Collection {
  let db = client.database("main");
  db.collection("transactions")
}
