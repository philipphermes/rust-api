use mongodb::results::{InsertOneResult};
use mongodb::bson::oid::ObjectId;

pub fn extract_from_insert_one_result(result: Result<InsertOneResult, mongodb::bson::extjson::de::Error>) -> Result<ObjectId, &'static str> {
    let id_json = match result {
        Ok(created_category) => serde_json::to_value(created_category.inserted_id),
        Err(_err) => return Err("Insert error")
    };

    let value = match id_json {
        Ok(val) => val,
        Err(_err) => return Err("Json error")
    };

    let id = match value["$oid"].as_str() {
        None => return Err("Oid not found"),
        Some(oid) => oid,
    };

    let o_id = ObjectId::parse_str(id);

    match o_id {
        Ok(objet_id) => Ok(objet_id),
        Err(_err) => Err("Parse error")
    }
}