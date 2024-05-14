use actix_web::{get, web, HttpResponse, Responder};
use sqlx::{query_scalar, Error, PgPool};

#[get("/")]
pub async fn hello_world() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

#[get("/version")]
pub async fn get_version(pool: web::Data<PgPool>) -> impl Responder {
    tracing::info!("Getting version...");
    let result: Result<String, Error> = query_scalar("SELECT version()")
        .fetch_one(pool.get_ref())
        .await;

    match result {
        Ok(version) => HttpResponse::Ok().body(version),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {:?}", e)),
    }
}
