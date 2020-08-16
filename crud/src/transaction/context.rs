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
pub struct CreateTransactionContext {}

impl CreateTransactionContext {
  pub fn from(body: &CreateTransactionRequest) -> Result<Self, ContextError> {
    CreateTransactionContext::new()
      .build()
      .map_err(|_e| ContextError)
  }

  pub fn new() -> CreateTransactionContextBuilder {
    CreateTransactionContextBuilder::default()
  }
}