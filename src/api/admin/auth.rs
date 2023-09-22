use actix_web::{web::Data, HttpResponse, post};
use actix_web::web::{Json};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use mongodb::bson::{Uuid};
use mongodb::bson::extjson::de::Error;

use crate::repository::api_auth_repository::ApiAuthRepo;
use crate::repository::user_repository::UserRepo;
use crate::model::api_auth::ApiAuth;
use crate::storefront::auth::auth;

#[post("/admin-api/auth/create")]
pub async fn create_auth(user_repo: Data<UserRepo>, api_auth_repo: Data<ApiAuthRepo>, mut api_auth: Json<ApiAuth>, bearer_auth: BearerAuth) -> HttpResponse {
    let user_detail = auth(user_repo.clone(), bearer_auth.token()).await;

    let current_user = match user_detail {
        Ok(user) => user,
        Err(_err) => return HttpResponse::Unauthorized().json("User not found"),
    };

    if !current_user.roles.contains(&"ROLE_ADMIN".to_string()) {
        return HttpResponse::Unauthorized().json("Not a Admin");
    }

    api_auth.token = Option::from(Uuid::new().to_string());

    let api_auth_create = api_auth_repo.create_auth(api_auth.clone()).await;

    match api_auth_create {
        Ok(_) => {},
        Err(err) => return HttpResponse::InternalServerError().json(err.to_string()),
    };

    let auth = api_auth_repo.get_api_auth_by_token(api_auth.token.clone().unwrap().as_str()).await;

    match auth {
        Ok(auth_res) => HttpResponse::Ok().json(auth_res),
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}

pub async fn api_auth(api_auth_repo: Data<ApiAuthRepo>, token: &str) -> Result<ApiAuth, Error> {
    api_auth_repo.get_api_auth_by_token(token).await
}