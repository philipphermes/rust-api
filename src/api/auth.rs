use actix_web::{web::Data, HttpResponse, post};
use actix_web::web::{Json};
use mongodb::bson::Uuid;
use pwhash::bcrypt;
use serde::{Serialize, Deserialize};

use crate::repository::user_repository::UserRepo;
use crate::model::user::User;

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    token: String,
}

#[post("/auth/login")]
pub async fn login(user_repo: Data<UserRepo>, auth_user: Json<User>) -> HttpResponse {
    let user_detail = user_repo.get_user_by_email(auth_user.email.as_str()).await;

    let current_user: User = match user_detail {
        Ok(user) => user,
        Err(_err) => return HttpResponse::Unauthorized().json("User not found"),
    };

    if !bcrypt::verify("password", current_user.password.as_str()) {
        return HttpResponse::Unauthorized().json("Invalid credentials");
    }

    let user_update = user_repo.update_token(current_user.id, Uuid::new().to_string()).await;
    let user_detail = user_repo.get_user_by_email(auth_user.email.as_str()).await;

    match user_detail {
        Ok(user) => HttpResponse::Ok().json(user.token),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/auth")]
pub async fn auth(user_repo: Data<UserRepo>, token: Json<Token>) -> HttpResponse {
    let user_detail = user_repo.get_user_by_token(token.token.as_str()).await;

    let current_user: User = match user_detail {
        Ok(user) => user,
        Err(_err) => return HttpResponse::Unauthorized().json("Invalid credentials"),
    };

    if current_user.token != "" && token.token == current_user.token {
        return HttpResponse::Ok().json(current_user.token)
    } else {
        HttpResponse::Unauthorized().json("Invalid credentials")
    }
}


#[post("/auth/logout")]
pub async fn logout(user_repo: Data<UserRepo>, token: Json<Token>) -> HttpResponse {
    let user_detail = user_repo.get_user_by_token(token.token.as_str()).await;

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