use actix_web::{web::Data, HttpResponse, post, HttpRequest, get, patch, delete};
use actix_web::web::{Json};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use mongodb::bson::Uuid;

use crate::repository::category_repository::CategoryRepo;
use crate::repository::api_auth_repository::ApiAuthRepo;
use crate::model::category::Category;
use crate::admin::auth::api_auth;

#[post("/admin-api/category")]
pub async fn create_category(
    api_auth_repo: Data<ApiAuthRepo>,
    category_repo: Data<CategoryRepo>,
    mut category: Json<Category>,
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

    let category_id: String = Uuid::new().to_string();
    category.id = Option::from(category_id.clone());

    for (p_key, product) in category.clone().products.iter().enumerate() {
        category.products[p_key].id = Option::from(Uuid::new().to_string());

        for (v_key, variant) in product.clone().variants.iter().enumerate() {
            category.products[p_key].variants[v_key].id = Option::from(Uuid::new().to_string());
        }
    }

    let category_create = category_repo.create_category(category.clone()).await;

    match category_create {
        Ok(_) => {}
        Err(err) => return HttpResponse::Unauthorized().json(err.to_string())
    }

    let created_category = category_repo.get_by_id(category_id.clone()).await;

    match created_category {
        Ok(created_category_res) => HttpResponse::Ok().json(created_category_res),
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}

#[patch("/admin-api/category")]
pub async fn patch_category(
    api_auth_repo: Data<ApiAuthRepo>,
    category_repo: Data<CategoryRepo>,
    category: Json<Category>,
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

    if (category.id.is_none()) {
        return HttpResponse::BadRequest().json("Id is required")
    }

    let mut existing_category = match category_repo.get_by_id(category.clone().id.unwrap()).await {
        Ok(category_res) => category_res,
        Err(err) => return HttpResponse::NotFound().json(err.to_string()),
    };

    existing_category.description = category.clone().description;

    for (p_key, existing_product) in existing_category.clone().products.iter().enumerate() {
        for product in category.clone().products {
            if existing_product.id == product.id {
                existing_category.products[p_key].description = product.description.clone();
                existing_category.products[p_key].sku = product.sku.clone();

                for (v_key, existing_variant) in existing_product.clone().variants.iter().enumerate() {
                    for variant in product.clone().variants {
                        if existing_variant.id == variant.id {
                            existing_category.products[p_key].variants[v_key].description = variant.description.clone();
                            existing_category.products[p_key].variants[v_key].sku = variant.sku.clone();
                            existing_category.products[p_key].variants[v_key].price = variant.price.clone();
                            existing_category.products[p_key].variants[v_key].sale_price = variant.sale_price.clone();
                            existing_category.products[p_key].variants[v_key].image_url = variant.image_url.clone();
                        }
                    }
                }
            }
        }
    }

    let update_res = category_repo.update_category(existing_category.clone()).await;

    match update_res {
        Ok(_) => HttpResponse::Ok().json(existing_category),
        Err(err) => HttpResponse::InternalServerError().json(err.to_string())
    }
}

#[delete("/admin-api/category/{id}")]
pub async fn delete_category(req: HttpRequest, api_auth_repo: Data<ApiAuthRepo>, category_repo: Data<CategoryRepo>, bearer_auth: BearerAuth) -> HttpResponse {
    let api_auth = api_auth(api_auth_repo, bearer_auth.token()).await;

    let auth = match api_auth {
        Ok(auth) => auth,
        Err(_err) => return HttpResponse::Unauthorized().json("Unauthorized"),
    };

    if !auth.scopes.contains(&"read_category".to_string()) && !auth.scopes.contains(&"write_category".to_string()) {
        return HttpResponse::Unauthorized().json("Scopes read_category and write_category are required");
    }

    let id: String = req.match_info().get("id").unwrap().parse().unwrap();
    let result = category_repo.delete_category(id).await;

    match result {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}

#[get("/admin-api/category/{id}")]
pub async fn get_category(req: HttpRequest, api_auth_repo: Data<ApiAuthRepo>, category_repo: Data<CategoryRepo>, bearer_auth: BearerAuth) -> HttpResponse {
    let api_auth = api_auth(api_auth_repo, bearer_auth.token()).await;

    let auth = match api_auth {
        Ok(auth) => auth,
        Err(_err) => return HttpResponse::Unauthorized().json("Unauthorized"),
    };

    if !auth.scopes.contains(&"read_category".to_string()) {
        return HttpResponse::Unauthorized().json("Scope read_category is required");
    }

    let id: String = req.match_info().get("id").unwrap().parse().unwrap();
    let category = category_repo.get_by_id(id).await;

    match category {
        Ok(category_res) => HttpResponse::Ok().json(category_res),
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}
