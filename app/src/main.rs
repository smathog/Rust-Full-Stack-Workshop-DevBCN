mod errors;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use dotenvy::{dotenv_override, var};
use sqlx::PgPool;


#[get("/")]
async fn hello_world() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv_override()?;
    let username = var("USERNAME")?;
    let password = var("PASSWORD")?;
    println!("{}", format!("User: {} Pass: {}", username, password));

    let pool = PgPool::connect_lazy("placeholder")
        .map_err(|e| errors::SqlxErrorWrapper(e))?;
    HttpServer::new(move || App::new().app_data(pool.clone()).service(hello_world))
        .bind(("localhost", 8080))?
        .run()
        .await?;
    Ok(())
}
