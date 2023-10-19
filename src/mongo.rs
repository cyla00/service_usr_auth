#[allow(unused_imports)]
use mongodb::{Client, options::ClientOptions, error::Result, bson::doc, Database};

pub async fn database_connection() -> Database {
    let client = Client::with_uri_str("mongodb://localhost:27017").await.unwrap();
    let db = client.database("testDb");
    println!("database connected");
    return db
}