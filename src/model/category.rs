use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};
use crate::model::product::Product;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Category {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub description: String,
    pub products: Vec<Product>
}