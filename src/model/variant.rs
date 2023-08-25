use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Variant {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub sku: String,
    pub product_id: ObjectId,
    pub description: String,
    pub price: f32,
    pub sale_price: Option<f32>,
    pub image_url: String
}