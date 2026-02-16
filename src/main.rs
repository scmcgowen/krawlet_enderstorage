use actix_web::{App, HttpServer, middleware, web, HttpResponse};
use actix_cors::Cors;
mod routes;
mod models;
use crate::routes::config;
use std::env;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    tracing::info!("Starting server");
    dotenvy::dotenv().ok();
    let server_url = env::var("SERVER_URL").unwrap_or_else(|_| "0.0.0.0:8080".to_string());


    let http_server = HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .wrap(middleware::Logger::new(r#"%a "%r" %s %b "%{Referer}i" "%{User-Agent}i" "%{X-CC-ID}i" %T"#))
            .wrap(cors)
            .configure(config)
            .default_service(web::route().to(|| async { HttpResponse::NotFound().finish() }))
    })
    .bind(&server_url)?
    .run();
    let b = http_server.await;
    match b {
        Ok(_) => Ok(()),
        Err(e) => {
            tracing::error!("Server error: {}", e);
            Err(e.into())
        },
    }


}
