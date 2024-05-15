use actix_web::web;
use actix_web::web::ServiceConfig;

use endpoints::*;

pub fn films_service_dynamic(cfg: &mut ServiceConfig) {
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
    use crate::RepoTypeDynamic;
    use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
    use shared::models::{CreateFilmModel, FilmModel};
    use uuid::Uuid;

    #[get("")]
    async fn get_films(repo: RepoTypeDynamic) -> impl Responder {
        match repo.read().await.get_films().await {
            Ok(films) => HttpResponse::Ok().json(films),
            Err(error) => HttpResponse::NotFound().body(error),
        }
    }

    #[get("/{id}")]
    async fn get_film(id: web::Path<Uuid>, repo: RepoTypeDynamic) -> impl Responder {
        match repo.read().await.get_film(&id).await {
            Ok(film) => HttpResponse::Ok().json(film),
            Err(error) => HttpResponse::NotFound().body(error),
        }
    }

    #[post("")]
    async fn create_film(
        film: web::Json<CreateFilmModel>,
        repo: RepoTypeDynamic,
    ) -> impl Responder {
        match repo.write().await.create_film(&film).await {
            Ok(film) => HttpResponse::Ok().json(film),
            Err(error) => HttpResponse::InternalServerError().body(error),
        }
    }

    #[put("")]
    async fn update_film(film: web::Json<FilmModel>, repo: RepoTypeDynamic) -> impl Responder {
        match repo.write().await.update_film(&film).await {
            Ok(film) => HttpResponse::Ok().json(film),
            Err(error) => HttpResponse::NotFound().body(error),
        }
    }

    #[delete("/{id}")]
    async fn delete_film(id: web::Path<Uuid>, repo: RepoTypeDynamic) -> impl Responder {
        match repo.write().await.delete_film(&id).await {
            Ok(id) => HttpResponse::Ok().json(id),
            Err(error) => HttpResponse::NotFound().body(error),
        }
    }
}
