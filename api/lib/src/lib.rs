mod api;
mod film_repository;

pub use api::films::films_service;
pub use api::health::health_service;
pub use film_repository::FilmError;
pub use film_repository::FilmResult;
pub use film_repository::FilmRepository;
pub use film_repository::PostgresFilmRepository;