use actix_web::web::ServiceConfig;

pub fn health_service(cfg: &mut ServiceConfig) {
    cfg.service(endpoints::health);
}

mod endpoints {
    use actix_web::{get, HttpResponse, Responder};
    use crate::API_VERSION;

    #[get("/health")]
    async fn health() -> impl Responder {
        tracing::info!("Getting API version...");
        HttpResponse::Ok()
                .append_header(("version", API_VERSION))
                .body(API_VERSION)
    }
}
