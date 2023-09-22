use actix_web::{web::Data, HttpResponse, post, get, delete, patch};
use actix_web::web::{Json};
use actix_web_httpauth::extractors::bearer::BearerAuth;

use crate::repository::user_repository::UserRepo;
use crate::model::user::{User, UserOutput, UserUpdateCreate};
use crate::api::storefront::auth::auth;

#[get("/storefront/user")]
async fn get_user(user_repo: Data<UserRepo>, bearer_auth: BearerAuth) -> HttpResponse {
    let user_detail = auth(user_repo, bearer_auth.token()).await;

    match user_detail {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_err) => HttpResponse::Unauthorized().json("Invalid credentials"),
    }
}

#[delete("/storefront/user/delete")]
async fn delete_user(user_repo: Data<UserRepo>, bearer_auth: BearerAuth) -> HttpResponse {
    let user_detail = auth(user_repo.clone(), bearer_auth.token()).await;

    let current_user: User = match user_detail {
        Ok(user) => user,
        Err(_err) => return HttpResponse::Unauthorized().json("Invalid credentials"),
    };

    let user_delete = user_repo.delete_user(&current_user.id.unwrap()).await;

    match user_delete {
        Ok(_user) => HttpResponse::Ok().json("User was deleted"),
        Err(err) => HttpResponse::NotFound().body(err.to_string()),
    }
}

#[patch("/storefront/user/update")]
async fn update_user(user_repo: Data<UserRepo>, update_user: Json<UserUpdateCreate>, bearer_auth: BearerAuth) -> HttpResponse {
    let user_detail = auth(user_repo.clone(), bearer_auth.token()).await;

    match user_detail {
        Ok(_user) => (),
        Err(_err) => return HttpResponse::Unauthorized().json("Invalid credentials"),
    }

    let user_update = user_repo.update_user(update_user.clone(), bearer_auth.token()).await;

    match user_update {
        Ok(_user) => {},
        Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
    }

    let user = user_repo.get_user_by_token(bearer_auth.token()).await;

    match user {
        Ok(user_res) => HttpResponse::Ok().json(user_res),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string())
    }
}

#[post("/storefront/register")]
pub async fn register(user_repo: Data<UserRepo>, new_user: Json<UserUpdateCreate>) -> HttpResponse {
    let user_detail = user_repo.get_user_by_email(new_user.email.as_str()).await;

    if !user_detail.is_err() {
        return HttpResponse::Ok().json("Email already in use");
    }

    let user_register = user_repo.create_user(new_user.clone(), vec!["ROLE_USER".to_string(), "ROLE_ADMIN".to_string()]).await;

    match user_register {
        Ok(_user) => {},
        Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
    }

    let user = user_repo.get_user_by_email(new_user.email.as_str()).await;

    match user {
        Ok(user_res) => HttpResponse::Ok().json(user_res),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string())
    }
}

