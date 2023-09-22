use actix_web::{web::Data, HttpResponse, post, HttpRequest, get, patch, delete};
use actix_web::web::{Json};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use mongodb::bson::{doc, Uuid};

use crate::repository::category_repository::CategoryRepo;
use crate::repository::api_auth_repository::ApiAuthRepo;
use crate::admin::auth::api_auth;
use crate::model::category::Category;
use crate::model::product::Product;

#[post("/admin-api/category/{id}/product")]
pub async fn create_product(
    req: HttpRequest,
    api_auth_repo: Data<ApiAuthRepo>,
    category_repo: Data<CategoryRepo>,
    mut product: Json<Product>,
    bearer_auth: BearerAuth
) -> HttpResponse {
    let api_auth = api_auth(api_auth_repo, bearer_auth.token()).await;

    let auth = match api_auth {
        Ok(auth) => auth,
        Err(_err) => return HttpResponse::Unauthorized().json("Unauthorized"),
    };

    if !auth.scopes.contains(&"read_category".to_string()) && !auth.scopes.contains(&"write_category".to_string()) {
        return HttpResponse::Unauthorized().json("Scopes read_category and write_category are required");
    }

    let id: String = req.match_info().get("id").unwrap().parse().unwrap();

    let category_res = category_repo.get_by_id(id.clone()).await;

    let mut category: Category = match category_res {
        Ok(category) => category,
        Err(_err) => return HttpResponse::NotFound().json("")
    };

    product.id = Option::from(Uuid::new().to_string());

    category.products.push(product.clone());

    let prod_length: usize = category.clone().products.len() - 1;

    for (v_key, _variant) in product.clone().variants.iter().enumerate() {
        category.products[prod_length].variants[v_key].id = Option::from(Uuid::new().to_string());
    }

    let category_update = category_repo.update_category(category.clone()).await;

    match category_update {
        Ok(_) => {}
        Err(err) => return HttpResponse::InternalServerError().json(err.to_string())
    }

    let updated_category = category_repo.get_by_id(id.clone()).await;

    match updated_category {
        Ok(updated_category_res) => HttpResponse::Ok().json(updated_category_res),
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}