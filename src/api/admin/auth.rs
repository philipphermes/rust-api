use actix_web::{web::Data, HttpResponse, post};
use actix_web::web::{Json};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use mongodb::bson::{bson, Uuid};
use mongodb::bson::Bson::ObjectId;
use mongodb::bson::extjson::de::Error;
use serde_json::Value;

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

    let id_json = match api_auth_create {
        Ok(created_auth) => serde_json::to_value(created_auth.inserted_id),
        Err(err) => return HttpResponse::InternalServerError().json(err.to_string()),
    };

    let value = match id_json {
        Ok(val) => val,
        Err(err) => return HttpResponse::InternalServerError().json(err.to_string()),
    };

    let id = match value["$oid"].as_str() {
        None => return HttpResponse::InternalServerError().json("oid not found".to_string()),
        Some(oid) => oid,
    };

    let o_id = mongodb::bson::oid::ObjectId::parse_str(id);

    let obj_id = match o_id {
        Ok(objet_id) => objet_id,
        Err(_) => return HttpResponse::InternalServerError().json("Converting failed".to_string()),
    };

    let auth = api_auth_repo.get_api_auth_by_id(obj_id).await;

    match auth {
        Ok(auth_res) => HttpResponse::Ok().json(auth_res),
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}

pub async fn api_auth(api_auth_repo: Data<ApiAuthRepo>, token: &str) -> Result<ApiAuth, Error> {
    api_auth_repo.get_api_auth_by_token(token).await
}