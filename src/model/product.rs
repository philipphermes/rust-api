use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};
use crate::model::variant::Variant;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Product {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub category_id: ObjectId,
    pub description: String,
    pub variants: Vec<Variant>
}