mod errors;

use actix_web::{get, App, HttpResponse, HttpServer, Responder, web};
use dotenvy::{dotenv_override, var};
use sqlx::{Error, Executor, PgPool, query_scalar};
use tracing_actix_web::TracingLogger;

#[get("/")]
async fn hello_world() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

#[get("/version")]
async fn get_version(pool: web::Data<PgPool>) -> impl Responder {
    tracing::info!("Getting version...");
    let result: Result<String, Error> = query_scalar("SELECT version()")
        .fetch_one(pool.get_ref())
        .await;

    match result {
        Ok(version) => HttpResponse::Ok().body(version),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {:?}", e)),
    }
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up tracing subscriber
    let (writer, _guard) = tracing_appender::non_blocking(std::io::stdout());
    tracing_subscriber::fmt()
        .with_writer(writer)
        .with_level(true)
        .init();

    // Grab relevant environment variables to connect to postgres
    dotenv_override()?;
    let db_name = var("POSTGRES_DB_NAME")?;
    let db_host = var("POSTGRES_HOST")?;
    let db_port = var("POSTGRES_PORT")?;
    let db_user = var("POSTGRES_USERNAME")?;
    let db_pass = var("POSTGRES_PASSWORD")?;

    // Create sqlx DB pool for postgres, initialize DB if needed
    let connection_string = format!(
        "postgresql://{}:{}@{}:{}/{}",
        db_user, db_pass, db_host, db_port, db_name,
    );
    tracing::info!("Attempting to connect to DB to create pool...");
    let pool = PgPool::connect_lazy(&connection_string).map_err(|e| errors::SqlxErrorWrapper(e))?;
    tracing::info!("Created DB pool. Validating presence of schema...");
    pool.execute(include_str!("../../api/db/schema.sql"))
        .await
        .map_err(|e| errors::SqlxErrorWrapper(e))?;
    tracing::info!("Schema present...");

    // Startup actix backend server
    tracing::info!("Starting Actix HttpServer...");
    HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .app_data(web::Data::new(pool.clone()))
            .service(hello_world)
            .service(get_version)
    })
    .bind(("localhost", 8080))?
    .run()
    .await?;

    tracing::info!("Server shutdown complete.");
    Ok(())
}
