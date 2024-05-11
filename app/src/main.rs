use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello_world() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello_world))
        .bind(("localhost", 8080))?
        .run()
        .await
}
