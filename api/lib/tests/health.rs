use api_lib::health_service;

use actix_web::body::to_bytes;
use actix_web::http::StatusCode;
use actix_web::{test, App};
use dotenvy::{dotenv_override, var};
use sqlx::PgPool;

#[actix_web::test]
async fn test_health_check() {
    let app = test::init_service(App::new().configure(health_service)).await;
    let req = test::TestRequest::get().uri("/health").to_request();
    let resp = test::call_service(&app, req).await;
    let version = api_lib::API_VERSION;
    assert!(resp.status().is_success());
    assert_eq!(resp.status(), StatusCode::OK);
    let version_header = resp.headers().get("version").unwrap();
    assert_eq!(version_header, version);
    let body = to_bytes(resp.into_body()).await.unwrap();
    assert_eq!(std::str::from_utf8(&body).unwrap(), version);
}

fn get_pgpool() -> PgPool {
    // Grab relevant environment variables to connect to postgres
    dotenv_override().unwrap();
    let db_name = var("POSTGRES_DB_NAME").unwrap();
    let db_host = var("POSTGRES_HOST").unwrap();
    let db_port = var("POSTGRES_PORT").unwrap();
    let db_user = var("POSTGRES_USERNAME").unwrap();
    let db_pass = var("POSTGRES_PASSWORD").unwrap();

    // Create sqlx DB pool for postgres, initialize DB if needed
    let connection_string = format!(
        "postgresql://{}:{}@{}:{}/{}",
        db_user, db_pass, db_host, db_port, db_name,
    );

    PgPool::connect_lazy(&connection_string).unwrap()
}
