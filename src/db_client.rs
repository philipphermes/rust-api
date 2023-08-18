extern crate dotenv;

use std::env;
use dotenv::dotenv;
use mongodb::{Client, Database};

pub struct DbClient {
    pub db: Database,
}

impl DbClient {
    pub async fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGODB_URL") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable MONGODB_URL"),
        };
        let db_name = match env::var("MONGODB_DB") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable MONGODB_DB"),
        };

        let client = Client::with_uri_str(uri).await;
        let db = client.unwrap().database(&*db_name);

        DbClient { db }
    }
}