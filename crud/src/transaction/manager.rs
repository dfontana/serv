use mongodb::Client;
use std::error::Error;
use super::model::{Txid, Transaction};
use super::context::{CreateTransactionContext, ContextError};

#[derive(Builder)]
#[builder(setter(into, prefix = "with"))]
pub struct TransactionService {
  mongo: Client,
}

impl TransactionService {
  pub fn new() -> TransactionServiceBuilder {
    TransactionServiceBuilder::default()
  }
  pub fn create(&self, ctx: CreateTransactionContext) -> Result<Txid, Box<dyn Error>> {
    Err(ContextError.into())
  }
  pub fn load(&self, txid: Txid) -> Result<Transaction, Box<dyn Error>> {
    Err(ContextError.into())
  }
}


// use futures::stream::StreamExt;
// use mongodb::{
//   bson::{doc, Bson},
//   options::{ClientOptions, FindOptions},
//   Client,
// };

// async fn run() -> Result<(), mongodb::error::Error> {

//   // List the names of the databases in that deployment.
//   for db_name in client.list_database_names(None, None).await? {
//     println!("{}", db_name);
//   }

//   // Get a handle to a database.
//   let db = client.database("mydb");

//   // List the names of the collections in that database.
//   for collection_name in db.list_collection_names(None).await? {
//     println!("{}", collection_name);
//   }

//   // Get a handle to a collection in the database.
//   let collection = db.collection("books");

//   let docs = vec![
//     doc! { "title": "1984", "author": "George Orwell" },
//     doc! { "title": "Animal Farm", "author": "George Orwell" },
//     doc! { "title": "The Great Gatsby", "author": "F. Scott Fitzgerald" },
//   ];

//   // Insert some documents into the "mydb.books" collection.
//   collection.insert_many(docs, None).await?;

//   let filter = doc! { "author": "George Orwell" };
//   let find_options = FindOptions::builder().sort(doc! { "title": 1 }).build();
//   let mut cursor = collection.find(filter, find_options).await?;

//   // Iterate over the results of the cursor.
//   while let Some(result) = cursor.next().await {
//     match result {
//       Ok(document) => {
//         if let Some(title) = document.get("title").and_then(Bson::as_str) {
//           println!("title: {}", title);
//         } else {
//           println!("no title found");
//         }
//       }
//       Err(e) => return Err(e.into()),
//     }
//   }
//   Ok(())
// }