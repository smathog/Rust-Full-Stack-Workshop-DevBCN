mod errors;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use dotenvy::{dotenv_override, var};
use sqlx::{Executor, PgPool};

#[get("/")]
async fn hello_world() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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
    let pool = PgPool::connect_lazy(&connection_string).map_err(|e| errors::SqlxErrorWrapper(e))?;
    println!("Created DB pool...");
    pool.execute(include_str!("../../api/db/schema.sql"))
        .await
        .map_err(|e| errors::SqlxErrorWrapper(e))?;
    println!("Schema present...");

    // Startup actix backend server
    println!("Starting Actix HttpServer...");
    HttpServer::new(move || App::new().app_data(pool.clone()).service(hello_world))
        .bind(("localhost", 8080))?
        .run()
        .await?;
    Ok(())
}
