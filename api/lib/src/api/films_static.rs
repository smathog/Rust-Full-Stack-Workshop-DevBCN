use actix_web::web;
use actix_web::web::ServiceConfig;

use endpoints::*;

pub fn films_service_static(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/v1/films")
            .service(get_films)
            .service(get_film)
            .service(create_film)
            .service(update_film)
            .service(delete_film),
    );
}

mod endpoints {
    use actix_web::{delete, get, post, put, HttpResponse, Responder};

    #[get("")]
    async fn get_films() -> impl Responder {
        HttpResponse::Ok().finish()
    }

    #[get("/{id}")]
    async fn get_film() -> impl Responder {
        HttpResponse::Ok().finish()
    }

    #[post("")]
    async fn create_film() -> impl Responder {
        HttpResponse::Ok().finish()
    }

    #[put("/{id}")]
    async fn update_film() -> impl Responder {
        HttpResponse::Ok().finish()
    }

    #[delete("/{id}")]
    async fn delete_film() -> impl Responder {
        HttpResponse::Ok().finish()
    }
}
