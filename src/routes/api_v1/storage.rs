use std::{error::Error};

use actix_web::{HttpResponse, get, web};

use crate::models::krawlet::krawlet_response::KrawletResponse;

#[get("/storage")]
async fn storage() -> Result<HttpResponse, Box<dyn Error>> {
    let krawlet_response = reqwest::get("https://api.krawlet.cc/v1/storage")
        .await?
        .json::<KrawletResponse>()
        .await?;
    let storages = krawlet_response.toEnderStorages();
    Ok(HttpResponse::Ok().json(storages))
}
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(storage);
}
