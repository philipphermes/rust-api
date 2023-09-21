extern crate dotenv;

use actix_web::web::{Data};
use mongodb::{bson::{extjson::de::Error, doc}, Collection, bson};
use mongodb::bson::oid::ObjectId;
use mongodb::results::{DeleteResult, InsertOneResult, UpdateResult};
use serde::de::Error as DefaultError;
use serde_json::Number;

use crate::db_client::DbClient;
use crate::model::category::Category;

pub struct CategoryRepo {
    pub col: Collection<Category>,
}

impl CategoryRepo {
    pub async fn init(db_client: Data<DbClient>) -> Self {
        let col: Collection<Category> = db_client.db.collection("Category");
        CategoryRepo { col }
    }

    pub async fn create_category(&self, new_category: Category) -> Result<InsertOneResult, Error> {
        let category = self
            .col
            .insert_one(new_category, None)
            .await;

        if category.is_err() {
            Err(DefaultError::custom(category.err().unwrap().to_string()))
        } else {
            Ok(category.ok().unwrap())
        }
    }

    pub async fn update_category(&self, update_category: Category) -> Result<UpdateResult, Error> {
        let category = self
            .col
            .update_one(
                doc! {"_id": update_category.id.clone()},
                doc! {"$set": bson::to_bson(&update_category).unwrap() },
                None
            )
            .await;

        if category.is_err() {
            Err(DefaultError::custom(category.err().unwrap().to_string()))
        } else {
            Ok(category.ok().unwrap())
        }
    }

    pub async fn delete_category(&self, id: String) -> Result<DeleteResult, Error> {
        let filter = doc! {"_id": id};
        let category = self
            .col
            .delete_one(filter, None)
            .await;

        if category.is_err() {
            Err(DefaultError::custom(category.err().unwrap().to_string()))
        } else {
            Ok(category.ok().unwrap())
        }
    }

    pub async fn get_by_id(&self, id: String) -> Result<Category, Error> {
        let filter = doc! {"_id": id};

        let category = self
            .col
            .find_one(filter, None)
            .await;

        if category.is_err() {
            Err(DefaultError::custom(category.err().unwrap().to_string()))
        } else {
            let category_data = category.ok().unwrap();

            if category_data.is_none() {
                return Err(DefaultError::custom("Category not found"));
            }

            Ok(category_data.unwrap())
        }
    }
}

