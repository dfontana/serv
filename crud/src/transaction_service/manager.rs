use super::model::{
  Cents, PaymentDestination, PaymentSource, Transaction, TransactionBuilder, Txid,
};
use crate::errors::UserError;
use mongodb::{
  bson::{doc, from_bson, to_bson, oid::ObjectId},
  Collection,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CreateTransactionContext {
  pub cents: Cents,
  pub from: PaymentSource,
  pub to: PaymentDestination,
}

#[derive(Clone)]
pub struct TransactionService {
  txns: Collection,
}

impl TransactionService {
  pub fn new(txns: Collection) -> TransactionService {
    TransactionService { txns }
  }

  pub async fn create(&self, ctx: CreateTransactionContext) -> Result<Txid, UserError> {
    let txn = TransactionBuilder::default()
      .cents(ctx.cents.clone())
      .from(ctx.from.clone())
      .to(ctx.to.clone())
      .build()?;
    let doc = to_bson(&txn)?
      .as_document()
      .ok_or("Failed to build doc")?
      .to_owned();
    self
      .txns
      .insert_one(doc, None)
      .await
      .map(|res| Txid {
        id: res.inserted_id.as_object_id().unwrap().to_string(),
      })
      .map_err(UserError::from)
  }

  pub async fn load(&self, txid: Txid) -> Result<Transaction, UserError> {
    let filter = doc! {"_id": ObjectId::with_string(txid.id.as_str()).unwrap() };
    let maybe_doc = match self.txns.find_one(filter, None).await {
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
