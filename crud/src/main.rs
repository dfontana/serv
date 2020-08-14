use futures::stream::StreamExt;
use mongodb::{
  bson::{doc, Bson},
  options::{ClientOptions, FindOptions},
  Client,
};

#[tokio::main]
async fn main() {
  match run().await {
    Ok(_) => println!("meow"),
    Err(_) => println!("wood"),
  };
}

async fn run() -> Result<(), mongodb::error::Error> {
  let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
  client_options.app_name = Some("My App".to_string());
  let client = Client::with_options(client_options)?;

  // List the names of the databases in that deployment.
  for db_name in client.list_database_names(None, None).await? {
    println!("{}", db_name);
  }

  // Get a handle to a database.
  let db = client.database("mydb");

  // List the names of the collections in that database.
  for collection_name in db.list_collection_names(None).await? {
    println!("{}", collection_name);
  }

  // Get a handle to a collection in the database.
  let collection = db.collection("books");

  let docs = vec![
    doc! { "title": "1984", "author": "George Orwell" },
    doc! { "title": "Animal Farm", "author": "George Orwell" },
    doc! { "title": "The Great Gatsby", "author": "F. Scott Fitzgerald" },
  ];

  // Insert some documents into the "mydb.books" collection.
  collection.insert_many(docs, None).await?;

  let filter = doc! { "author": "George Orwell" };
  let find_options = FindOptions::builder().sort(doc! { "title": 1 }).build();
  let mut cursor = collection.find(filter, find_options).await?;

  // Iterate over the results of the cursor.
  while let Some(result) = cursor.next().await {
    match result {
      Ok(document) => {
        if let Some(title) = document.get("title").and_then(Bson::as_str) {
          println!("title: {}", title);
        } else {
          println!("no title found");
        }
      }
      Err(e) => return Err(e.into()),
    }
  }
  Ok(())
}
