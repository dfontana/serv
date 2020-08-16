use super::model::{Cents, PaymentDestination, PaymentSource};
use super::CreateTransactionRequest;
use super::UserError;
use std::{error::Error, fmt};

#[derive(Debug, Clone)]
pub struct ContextError;

impl Error for ContextError {}
impl fmt::Display for ContextError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Failure to build Context from Request")
  }
}

impl From<ContextError> for UserError {
  fn from(item: ContextError) -> Self {
    UserError::ValidationError {
      field: "Context".into(),
      reason: item.to_string(),
    }
  }
}
impl From<Box<ContextError>> for UserError {
  fn from(item: Box<ContextError>) -> Self {
    UserError::ValidationError {
      field: "Context".into(),
      reason: item.to_string(),
    }
  }
}

#[derive(Builder, Clone, Debug)]
#[builder(setter(into, prefix = "with"))]
pub struct CreateTransactionContext {
  pub cents: Cents,
  pub from: PaymentSource,
  pub to: PaymentDestination,
}

impl CreateTransactionContext {
  pub fn from(body: &CreateTransactionRequest) -> Result<Self, ContextError> {
    CreateTransactionContext::new()
      .with_cents(body.cents.clone())
      .with_from(body.from.clone())
      .with_to(body.to.clone())
      .build()
      .map_err(|_e| ContextError)
  }

  pub fn new() -> CreateTransactionContextBuilder {
    CreateTransactionContextBuilder::default()
  }
}
