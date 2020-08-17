use crate::transaction_service::{CreateTransactionContext, Txid};
use actix_web::{web, HttpResponse, Responder};

pub fn init(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::scope("/txn")
      .service(
        web::resource("")
          .route(web::post().to(create_transaction))
          .route(web::get().to(search_transactions)),
      )
      .service(web::resource("/{id}").route(web::get().to(load_transaction))),
  );
}

async fn create_transaction(
  app: web::Data<crate::AppState>,
  body: web::Json<CreateTransactionContext>,
) -> impl Responder {
  let ctx = body.into_inner();
  match app.service_manager.txn.create(ctx).await {
    Ok(txid) => HttpResponse::Ok().json(txid),
    Err(e) => {
      println!("Error, {:?}", e);
      e.response()
    }
  }
}

async fn load_transaction(
  app: web::Data<crate::AppState>,
  path: web::Path<String>,
) -> impl Responder {
  let txid = Txid {
    id: path.into_inner(),
  };
  match app.service_manager.txn.load(txid).await {
    Ok(transaction) => HttpResponse::Ok().json(transaction),
    Err(e) => e.response(),
  }
}

async fn search_transactions(_mgr: web::Data<crate::AppState>) -> impl Responder {
  HttpResponse::Ok()
}
