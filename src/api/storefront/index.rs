use actix_web::{get, Responder, Result};
use serde::{Serialize};
use actix_web::web::{Json};

#[derive(Serialize)]
struct Routes {
    routes: Vec<String>
}

#[get("/storefront")]
async fn index() -> Result<impl Responder> {
    let obj = Routes {
        routes: vec![
            "/storefront/login".to_string(),
            "/storefront/logout".to_string(),
            "/storefront/user/register".to_string(),
            "/storefront/user/update".to_string(),
            "storefront/user/delete".to_string(),
            "/storefront/user".to_string(),
        ]
    };
    Ok(Json(obj))
}
