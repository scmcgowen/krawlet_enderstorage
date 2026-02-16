mod api_v1;
use actix_web::{HttpResponse, web, middleware, get};


#[get("/")]
pub async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello, world!")
}

pub fn config (cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .wrap(middleware::NormalizePath::trim())
            .configure(api_v1::config)
    );
    cfg.service(web::scope("").service(index));
}
