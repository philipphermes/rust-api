use actix_web::{get, Responder, Result};
use serde::{Serialize};
use actix_web::web::{Json};

#[derive(Serialize)]
struct Message {
    message: String,
}

#[get("/")]
async fn index() -> Result<impl Responder> {
    let obj = Message {
        message: "Hello".to_string(),
    };
    Ok(Json(obj))
}
