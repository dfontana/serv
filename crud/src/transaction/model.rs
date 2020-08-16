use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Txid {
  #[serde(flatten, rename = "_id")]
  pub id: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Cents {
  #[serde(flatten)]
  pub val: i32,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct User {
  pub id: u32,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum PaymentSource {
  User(User),
  CreditCard,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum PaymentDestination {
  User(User),
  Bank,
}

#[derive(Deserialize, Serialize, Builder, Clone, Debug)]
#[builder(setter(into))]
pub struct Transaction {
  id: Option<Txid>,
  cents: Cents,
  from: PaymentSource,
  to: PaymentDestination,
}
