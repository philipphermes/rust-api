extern crate dotenv;
extern crate core;

mod db_client;
mod model;
mod repository;
mod api;


use actix_web::{web::Data, get, Responder, Result, App, HttpServer};
use serde::{Serialize};
use actix_web::web::{Json};

use crate::db_client::DbClient;
use crate::repository::user_repository::UserRepo;
use crate::api::user::{create_user, get_user};


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


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = DbClient::init().await;
    let db_data = Data::new(db);

    let user_repository = UserRepo::init(db_data.clone()).await;
    let user_repository_data = Data::new(user_repository);

    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .app_data(user_repository_data.clone())
            .service(index)
            .service(get_user)
            .service(create_user)
    })
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}
