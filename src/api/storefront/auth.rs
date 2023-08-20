use actix_web::{web::Data, HttpResponse, post};
use actix_web::web::{Json};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use mongodb::bson::Uuid;
use pwhash::bcrypt;
use mongodb::bson::extjson::de::Error;

use crate::repository::user_repository::UserRepo;
use crate::model::user::User;
use crate::model::user::UserUpdateCreate;

#[post("/storefront/login")]
pub async fn login(user_repo: Data<UserRepo>, auth_user: Json<UserUpdateCreate>) -> HttpResponse {
    let user_detail = user_repo.get_user_by_email(auth_user.email.as_str()).await;

    let current_user: User = match user_detail {
        Ok(user) => user,
        Err(_err) => return HttpResponse::Unauthorized().json("User not found"),
    };

    if !bcrypt::verify("password", current_user.password.as_str()) {
        return HttpResponse::Unauthorized().json("Invalid credentials");
    }

    let user_update = user_repo.update_token(current_user.id, Uuid::new().to_string()).await;

    match user_update {
        Ok(_user) => (),
        Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
    }

    let user_detail = user_repo.get_user_by_email(auth_user.email.as_str()).await;

    match user_detail {
        Ok(user) => HttpResponse::Ok().json(user.token),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/storefront/logout")]
pub async fn logout(user_repo: Data<UserRepo>, bearer_auth: BearerAuth) -> HttpResponse {
    let user_detail = auth(user_repo.clone(), bearer_auth.token()).await;

    let current_user: User = match user_detail {
        Ok(user) => user,
        Err(_err) => return HttpResponse::Unauthorized().json("Invalid credentials"),
    };

    let user_detail = user_repo.update_token(current_user.id, "".to_string()).await;

    match user_detail {
        Ok(_user) => HttpResponse::Ok().json("logged out"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn auth(user_repo: Data<UserRepo>, token: &str) -> Result<User, Error> {
    user_repo.get_user_by_token(token).await
}