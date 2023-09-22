extern crate dotenv;

use actix_web::web::{Data};
use mongodb::{bson::{extjson::de::Error, doc, Uuid}, Collection};
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

    pub async fn create_auth(&self, mut new_auth: ApiAuth) -> Result<InsertOneResult, Error> {
        new_auth.id = Option::from(Uuid::new().to_string());

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

    pub async fn get_api_auth_by_token(&self, token: &str) -> Result<ApiAuth, Error> {
        let filter = doc! {"token": token.to_string()};

        let auth = self
            .col
            .find_one(filter, None)
            .await;

        if auth.is_err() {
            Err(DefaultError::custom(auth.err().unwrap().to_string()))
        } else {
            let auth_data = auth.ok().unwrap();

            if auth_data.is_none() {
                return Err(DefaultError::custom("Auth not found"));
            }

            Ok(auth_data.unwrap())
        }
    }

    pub async fn get_api_auth_by_id(&self, id: String) -> Result<ApiAuth, Error> {
        let filter = doc! {"_id": id};

        let auth = self
            .col
            .find_one(filter, None)
            .await;

        if auth.is_err() {
            Err(DefaultError::custom(auth.err().unwrap().to_string()))
        } else {
            let auth_data = auth.ok().unwrap();

            if auth_data.is_none() {
                return Err(DefaultError::custom("Auth not found"));
            }

            Ok(auth_data.unwrap())
        }
    }
}

