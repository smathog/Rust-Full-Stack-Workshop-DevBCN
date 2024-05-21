use crate::API_VERSION;
use actix_web::web::ServiceConfig;
use actix_web::{get, HttpResponse, Responder};

pub fn health_service(cfg: &mut ServiceConfig) {
    cfg.service(health);
}
#[get("/health")]
async fn health() -> impl Responder {
    tracing::info!("Getting API version...");
    HttpResponse::Ok()
        .append_header(("version", API_VERSION))
        .body(API_VERSION)
}
