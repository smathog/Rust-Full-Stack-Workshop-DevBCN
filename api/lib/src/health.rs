use actix_web::web::ServiceConfig;

use endpoints::*;

pub fn health_service(cfg: &mut ServiceConfig) {
    cfg.service(health);
}

mod endpoints {
    use actix_web::{get, web, HttpResponse, Responder};
    use sqlx::{query_scalar, Error, PgPool};

    #[get("/health")]
    async fn health(pool: web::Data<PgPool>) -> impl Responder {
        tracing::info!("Getting version...");
        let result: Result<String, Error> = query_scalar("SELECT version()")
            .fetch_one(pool.get_ref())
            .await;

        match result {
            Ok(version) => HttpResponse::Ok()
                .append_header(("version", version.clone()))
                .body(version),
            Err(e) => HttpResponse::InternalServerError().body(format!("Error: {:?}", e)),
        }
    }
}