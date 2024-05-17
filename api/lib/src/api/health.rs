use actix_web::{get, HttpResponse, Responder};
use actix_web::web::ServiceConfig;
use crate::API_VERSION;

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
