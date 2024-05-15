use actix_web::{web, App, HttpServer};
use api_lib::{FilmRepository, PostgresFilmRepository, RepoTypeDynamic};
use dotenvy::{dotenv_override, var};
use sqlx::{Executor, PgPool};
use tokio::sync::RwLock;
use tracing_actix_web::TracingLogger;

pub mod errors;

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
    let global_state: RepoTypeDynamic = web::Data::new(RwLock::new(
        Box::new(PostgresFilmRepository::new(pool.clone())),
    ));

    // Startup actix backend server
    tracing::info!("Starting Actix HttpServer...");
    HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .app_data(global_state.clone())
            .configure(api_lib::health_service)
            .configure(api_lib::films_service_dynamic)
    })
    .bind(("localhost", 8080))?
    .run()
    .await?;

    tracing::info!("Server shutdown complete.");
    Ok(())
}
