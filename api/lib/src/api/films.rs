use actix_web::web::{resource, Data, Json, Path, ServiceConfig};
use actix_web::{web, HttpResponse, Responder};
use shared::models::{CreateFilmModel, FilmModel};
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::FilmRepository;

pub fn films_service<T: FilmRepository>(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/v1/films")
            .service(resource("").get(get_films::<T>))
            .service(resource("/{id}").get(get_film::<T>))
            .service(resource("").post(create_film::<T>))
            .service(resource("").put(update_film::<T>))
            .service(resource("/{id}").delete(delete_film::<T>)),
    );
}

async fn get_films<T: FilmRepository>(repo: Data<RwLock<T>>) -> impl Responder {
    match repo.read().await.get_films().await {
        Ok(films) => HttpResponse::Ok().json(films),
        Err(err) => HttpResponse::InternalServerError().body(err),
    }
}

async fn get_film<T: FilmRepository>(id: Path<Uuid>, repo: Data<RwLock<T>>) -> impl Responder {
    match repo.read().await.get_film(&id).await {
        Ok(film) => HttpResponse::Ok().json(film),
        Err(err) => HttpResponse::NotFound().body(err),
    }
}

async fn create_film<T: FilmRepository>(
    film: Json<CreateFilmModel>,
    repo: Data<RwLock<T>>,
) -> impl Responder {
    match repo.write().await.create_film(&film).await {
        Ok(film) => HttpResponse::Ok().json(film),
        Err(err) => HttpResponse::InternalServerError().body(err),
    }
}

async fn update_film<T: FilmRepository>(
    film: Json<FilmModel>,
    repo: Data<RwLock<T>>,
) -> impl Responder {
    match repo.write().await.update_film(&film).await {
        Ok(film) => HttpResponse::Ok().json(film),
        Err(err) => HttpResponse::NotFound().body(err),
    }
}

async fn delete_film<T: FilmRepository>(path: Path<Uuid>, repo: Data<RwLock<T>>) -> impl Responder {
    match repo.write().await.delete_film(&path).await {
        Ok(id) => HttpResponse::Ok().json(id),
        Err(err) => HttpResponse::NotFound().body(err),
    }
}
