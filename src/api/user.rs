use actix_web::{web::Data, HttpResponse, post, get};
use actix_web::web::{Json, Path};
use mongodb::bson::Uuid;

use crate::repository::user_repository::UserRepo;
use crate::model::user::User;

#[get("/user/{email}")]
async fn get_user(user_repo: Data<UserRepo>, path: Path<String>) -> HttpResponse {
    let email = path.into_inner();

    let user_detail = user_repo.get_user_by_email(&email).await;

    match user_detail {
        Ok(course) => HttpResponse::Ok().json(course),
        Err(err) => HttpResponse::NotFound().body(err.to_string()),
    }
}

#[post("/user/create")]
pub async fn create_user(user_repo: Data<UserRepo>, new_user: Json<User>) -> HttpResponse {
    let user_detail = user_repo.get_user_by_email(new_user.email.as_str()).await;

    if !user_detail.is_err() {
        return HttpResponse::Ok().json("Email already in use");
    }

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

