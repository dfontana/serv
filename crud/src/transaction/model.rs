use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Txid {
  id: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Cents {
  #[serde(flatten)]
  val: i32,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct User {
  id: u32,
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

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Transaction {
  id: Txid,
  cents: Cents,
  from: PaymentSource,
  to: PaymentDestination,
}