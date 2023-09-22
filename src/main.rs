extern crate dotenv;
extern crate core;

mod db_client;
mod model;
mod repository;
mod api;

mod service;

use actix_web::{web::Data, App, HttpServer};

use crate::db_client::DbClient;
use crate::repository::user_repository::UserRepo;
use crate::repository::api_auth_repository::ApiAuthRepo;
use crate::repository::category_repository::CategoryRepo;
use crate::api::storefront;
use crate::api::admin;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = DbClient::init().await;
    let db_data = Data::new(db);

    let user_repository = UserRepo::init(db_data.clone()).await;
    let user_repository_data = Data::new(user_repository);

    let api_auth_repository = ApiAuthRepo::init(db_data.clone()).await;
    let api_auth_repository_data = Data::new(api_auth_repository);

    let category_repository = CategoryRepo::init(db_data.clone()).await;
    let category_repository_data = Data::new(category_repository);

    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .app_data(user_repository_data.clone())
            .app_data(api_auth_repository_data.clone())
            .app_data(category_repository_data.clone())
            .service(storefront::auth::login)
            .service(storefront::auth::logout)
            .service(storefront::index::index)
            .service(storefront::user::get_user)
            .service(storefront::user::register)
            .service(storefront::user::delete_user)
            .service(storefront::user::update_user)
            .service(admin::auth::create_auth)
            .service(admin::category::create_category)
            .service(admin::category::patch_category)
            .service(admin::category::delete_category)
            .service(admin::category::get_category)
            .service(admin::category::get_categories)
            .service(admin::product::create_product)
    })
        .bind(("127.0.0.1", 3000))?
        .run()
        .await
}
