extern crate dotenv;

use actix_web::web::{Data};
use mongodb::{bson::{extjson::de::Error, doc, Uuid}, Collection};
use mongodb::results::{DeleteResult, InsertOneResult, UpdateResult};
use serde::de::Error as DefaultError;
use pwhash::bcrypt;

use crate::db_client::DbClient;
use crate::model::user::User;
use crate::model::user::UserUpdateCreate;

pub struct UserRepo {
    pub col: Collection<User>,
}

impl UserRepo {
    pub async fn init(db_client: Data<DbClient>) -> Self {
        let col: Collection<User> = db_client.db.collection("User");
        UserRepo { col }
    }

    pub async fn create_user(&self, new_user: UserUpdateCreate, roles: Vec<String>) -> Result<InsertOneResult, Error> {
        let user = User {
            id: Option::from(Uuid::new().to_string()),
            email: new_user.email,
            password: bcrypt::hash(new_user.password.to_string()).unwrap(),
            token: None,
            roles
        };

        let user = self
            .col
            .insert_one(user, None)
            .await;

        if user.is_err() {
            Err(DefaultError::custom(user.err().unwrap().to_string()))
        } else {
            Ok(user.ok().unwrap())
        }
    }

    pub async fn update_user(&self, update_user: UserUpdateCreate, token: &str) -> Result<UpdateResult, Error> {
        let filter = doc! {"token": token};
        let new_user = doc! {
                "$set":
                    {
                        "email": update_user.email,
                        "password": bcrypt::hash(update_user.password.to_string()).unwrap(),
                    },
            };
        let updated_user = self
            .col
            .update_one(filter, new_user, None)
            .await;

        if updated_user.is_err() {
            Err(DefaultError::custom(updated_user.err().unwrap().to_string()))
        } else {
            Ok(updated_user.ok().unwrap())
        }
    }

    pub async fn update_token(&self, id: String, token: String) -> Result<UpdateResult, Error> {
        let filter = doc! {"_id": id};
        let new_user = doc! {
                "$set":
                    {
                        "token": token
                    },
            };
        let updated_user = self
            .col
            .update_one(filter, new_user, None)
            .await;

        if updated_user.is_err() {
            Err(DefaultError::custom(updated_user.err().unwrap().to_string()))
        } else {
            Ok(updated_user.ok().unwrap())
        }
    }

    pub async fn delete_user(&self, id: &String) -> Result<DeleteResult, Error> {
        let filter = doc! {"_id": id};
        let user_detail = self
            .col
            .delete_one(filter, None)
            .await;

        if user_detail.is_err() {
            Err(DefaultError::custom(user_detail.err().unwrap().to_string()))
        } else {
            Ok(user_detail.ok().unwrap())
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
            let user_data = user.ok().unwrap();

            if user_data.is_none() {
                return Err(DefaultError::custom("User not found"));
            }

            Ok(user_data.unwrap())
        }
    }

    pub async fn get_user_by_token(&self, token: &str) -> Result<User, Error> {
        let filter = doc! {"token": token.to_string()};

        let user = self
            .col
            .find_one(filter, None)
            .await;

        if user.is_err() {
            Err(DefaultError::custom(user.err().unwrap().to_string()))
        } else {
            let user_data = user.ok().unwrap();

            if user_data.is_none() {
                return Err(DefaultError::custom("User not found"));
            }

            Ok(user_data.unwrap())
        }
    }
}

