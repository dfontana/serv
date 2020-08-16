use super::errors::UserError;
use actix_web::{web, HttpResponse, Responder};
use manager::{TransactionService};
use context::{CreateTransactionContext};
use model::{Cents, PaymentDestination, PaymentSource, Txid};
use serde::{Deserialize, Serialize};

use mongodb::Client;

mod manager;
mod model;
mod context;

pub fn config(cfg: &mut web::ServiceConfig, mongo_client: Client) {
  let mgr = TransactionService::new().with_mongo(mongo_client).build();
  cfg.service(
    web::scope("/txn")
      .data(mgr)
      .service(
        web::resource("")
          .route(web::post().to(create_transaction))
          .route(web::get().to(search_transactions)),
      )
      .service(web::resource("/{id}").route(web::get().to(load_transaction))),
  );
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CreateTransactionRequest {
  id: Option<Txid>,
  cents: Cents,
  from: PaymentSource,
  to: PaymentDestination,
}

async fn create_transaction(
  mgr: web::Data<TransactionService>,
  body: web::Json<CreateTransactionRequest>,
) -> Result<impl Responder, UserError> {
  let req = body.into_inner();
  let ctx = CreateTransactionContext::from(&req).map_err(|e| UserError::from(e))?;
  let new_txid = mgr.create(ctx).map_err(|e| UserError::from(e))?;
  Ok(HttpResponse::Ok().json(new_txid))
}

async fn load_transaction(
  mgr: web::Data<TransactionService>,
  path: web::Path<Txid>,
) ->  Result<impl Responder, UserError> {
  let txid = path.into_inner();
  let transaction = mgr.load(txid).map_err(|e| UserError::from(e))?;
  Ok(HttpResponse::Ok().json(transaction))
}

async fn search_transactions(mgr: web::Data<TransactionService>) -> impl Responder {
  HttpResponse::Ok()
}
