use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Txid {
  #[serde(rename = "_id")]
  pub id: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Cents {
  pub val: i32,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct User {
  pub id: i32,
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
  #[builder(setter(into, strip_option), default)]
  #[serde(skip_serializing_if = "Option::is_none")]
  id: Option<Txid>,
  cents: Cents,
  from: PaymentSource,
  to: PaymentDestination,
}
