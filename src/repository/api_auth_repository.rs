extern crate dotenv;

use actix_web::web::{Data};
use mongodb::{bson::{extjson::de::Error}, Collection};
use mongodb::results::{InsertOneResult};
use serde::de::Error as DefaultError;

use crate::db_client::DbClient;
use crate::model::api_auth::ApiAuth;

pub struct ApiAuthRepo {
    pub col: Collection<ApiAuth>,
}

impl ApiAuthRepo {
    pub async fn init(db_client: Data<DbClient>) -> Self {
        let col: Collection<ApiAuth> = db_client.db.collection("ApiAuth");
        ApiAuthRepo { col }
    }

    pub async fn create_auth(&self, new_auth: ApiAuth) -> Result<InsertOneResult, Error> {
        let api_auth = self
            .col
            .insert_one(new_auth, None)
            .await;

        if api_auth.is_err() {
            Err(DefaultError::custom(api_auth.err().unwrap().to_string()))
        } else {
            Ok(api_auth.ok().unwrap())
        }
    }
}

