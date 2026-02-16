use actix_web::{web};
pub mod storage;
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.configure(storage::config);
}
