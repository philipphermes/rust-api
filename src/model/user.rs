use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub email: String,
    pub password: String,
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserOutput {
    pub id: Option<String>,
    pub email: String
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct UserUpdateCreate{
    pub id: Option<String>,
    pub email: String,
    pub password: String
}