use super::context::CreateTransactionContext;
use super::model::{Transaction, TransactionBuilder, Txid};
use super::UserError;
use mongodb::{
  bson::{doc, from_bson, to_bson},
  Client,
};

#[derive(Builder)]
#[builder(setter(into, prefix = "with"))]
pub struct TransactionService {
  mongo: Client,
}

impl TransactionService {
  pub fn new() -> TransactionServiceBuilder {
    TransactionServiceBuilder::default()
  }

  pub async fn create(&self, ctx: CreateTransactionContext) -> Result<Txid, UserError> {
    let txn = TransactionBuilder::default()
      .cents(ctx.cents.clone())
      .from(ctx.from.clone())
      .to(ctx.to.clone())
      .build();
    let doc = to_bson(&txn)?
      .as_document()
      .ok_or("Failed to build doc")?
      .to_owned();
    let collection = self.mongo.database("main").collection("transactions");
    collection
      .insert_one(doc, None)
      .await
      .map(|res| Txid {
        id: res.inserted_id.as_str().unwrap_or_default().to_string(),
      })
      .map_err(UserError::from)
  }

  pub async fn load(&self, txid: Txid) -> Result<Transaction, UserError> {
    let collection = self.mongo.database("main").collection("transactions");
    let filter = doc! {"_id": txid.id};
    let maybe_doc = match collection.find_one(filter, None).await {
      Ok(d) => d,
      Err(err) => {
        return Err(UserError::InternalError {
          cause: err.kind.to_string(),
        })
      }
    };
    let doc = maybe_doc.ok_or(UserError::ValidationError {
      field: "id".into(),
      reason: "Failed to find document".into(),
    })?;
    from_bson::<Transaction>(doc.into()).map_err(UserError::from)
  }
}
