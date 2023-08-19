use actix_web::{web::Data, HttpResponse, post, get, delete, patch};
use actix_web::web::{Json};
use actix_web_httpauth::extractors::bearer::BearerAuth;

use crate::repository::user_repository::UserRepo;
use crate::model::user::{User, UserOutput, UserUpdateCreate};
use crate::api::auth::auth;

#[get("/user")]
async fn get_user(user_repo: Data<UserRepo>, bearer_auth: BearerAuth) -> HttpResponse {
    let user_detail = auth(user_repo, bearer_auth.token()).await;

    match user_detail {
        Ok(user) => {
            let user_out = UserOutput {
                id: user.id,
                email: user.email
            };

            HttpResponse::Ok().json(user_out)
        },
        Err(_err) => HttpResponse::Unauthorized().json("Invalid credentials"),
    }
}

#[delete("/user/delete")]
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

#[patch("/user/update")]
async fn update_user(user_repo: Data<UserRepo>, update_user: Json<UserUpdateCreate>, bearer_auth: BearerAuth) -> HttpResponse {
    let user_detail = auth(user_repo.clone(), bearer_auth.token()).await;

    match user_detail {
        Ok(_user) => (),
        Err(_err) => return HttpResponse::Unauthorized().json("Invalid credentials"),
    }

    let user_update = user_repo.update_user(update_user.clone()).await;

    match user_update {
        Ok(_user) => HttpResponse::Ok().json("User was updated"),
        Err(err) => HttpResponse::NotFound().body(err.to_string()),
    }
}

#[post("/user/create")]
pub async fn create_user(user_repo: Data<UserRepo>, new_user: Json<UserUpdateCreate>) -> HttpResponse {
    let user_detail = user_repo.get_user_by_email(new_user.email.as_str()).await;

    if !user_detail.is_err() {
        return HttpResponse::Ok().json("Email already in use");
    }

    let user_detail = user_repo.create_user(new_user.clone()).await;

    match user_detail {
        Ok(_user) => HttpResponse::Ok().json(new_user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

