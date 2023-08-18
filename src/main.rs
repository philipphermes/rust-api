extern crate dotenv;
extern crate core;

mod db_client;
mod user;
mod user_repository;

mod models {}
mod repository {}

use dotenv::dotenv;
use actix_web::{web::Data, get, web, Responder, Result, App, HttpServer, HttpResponse, post};
use std::env;
use serde::{Deserialize, Serialize};
use crate::db_client::DbClient;
use crate::user_repository::UserRepo;
use actix_web::web::{Json, Path, Query};
use crate::user::User;
use mongodb::bson::Uuid;


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

#[get("/user/get/{email}")]
async fn get_user(user_repo: Data<UserRepo>, path: Path<String>) -> HttpResponse {
    let email = path.into_inner();

    let course_detail = user_repo.get_user_by_email(&email).await;

    match course_detail {
        Ok(course) => HttpResponse::Ok().json(course),
        Err(err) => HttpResponse::NotFound().body(err.to_string()),
    }
}

#[post("/user/create")]
pub async fn create_user(user_repo: Data<UserRepo>, new_user: Json<User>) -> HttpResponse {
    let data = User {
        id: Option::from(Uuid::new().to_string()),
        email: new_user.email.to_owned(),
        password: new_user.password.to_owned(),
    };
    let user_detail = user_repo.create_user(data.clone()).await;
    match user_detail {
        Ok(_user) => HttpResponse::Ok().json(data),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
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
