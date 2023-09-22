extern crate dotenv;

use actix_web::web::{Data};
use futures::TryStreamExt;
use mongodb::{bson::{extjson::de::Error, doc}, Collection, bson, Cursor};
use mongodb::bson::Document;
use mongodb::results::{DeleteResult, InsertOneResult, UpdateResult};
use serde::de::Error as DefaultError;

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

    pub async fn get_all(&self, filters: impl Into<Option<Document>>) -> Result<Vec<Category>, Error> {
        //TODO pagination
        let categories = self
            .col
            .find( filters, None)
            .await;

        if categories.is_err() {
            return Err(DefaultError::custom(categories.err().unwrap().to_string()));
        }

        let mut categories_cursor: Cursor<Category> = categories.ok().unwrap();

        let mut categories: Vec<Category> = Vec::new();

        while let Some(category) = categories_cursor
            .try_next()
            .await
            .ok()
            .expect("Error mapping through cursor")
        {
            categories.push(category)
        }
        Ok(categories)
    }
}

