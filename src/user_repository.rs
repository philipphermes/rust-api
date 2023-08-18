extern crate dotenv;

use actix_web::web::Data;
use mongodb::{bson::{extjson::de::Error, doc}, Collection};
use mongodb::results::InsertOneResult;
use serde::de::Error as DefaultError;
use crate::db_client::DbClient;
use crate::user::User;

pub struct UserRepo {
    pub col: Collection<User>,
}

impl UserRepo {
    pub async fn init(db_client: Data<DbClient>) -> Self {
        let col: Collection<User> = db_client.db.collection("User");
        UserRepo { col }
    }

    pub async fn create_user(&self, new_user: User) -> Result<InsertOneResult, Error> {
        let user = self
            .col
            .insert_one(new_user, None)
            .await;

        if user.is_err() {
            Err(DefaultError::custom(user.err().unwrap().to_string()))
        } else {
            Ok(user.ok().unwrap())
        }
    }


    pub async fn get_user_by_email(&self, email: &str) -> Result<User, Error> {
        let filter = doc! {"email": email.to_string()};

        let user = self
            .col
            .find_one(filter, None)
            .await;

        if user.is_err() {
            Err(DefaultError::custom(user.err().unwrap().to_string()))
        } else {
            Ok(user.ok().unwrap().unwrap())
        }
    }
}

