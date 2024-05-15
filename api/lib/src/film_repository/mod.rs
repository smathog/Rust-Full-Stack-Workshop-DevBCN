mod film_repository;
mod postgres_film_repository;
mod memory_film_repository;

pub use film_repository::FilmError;
pub use film_repository::FilmResult;
pub use film_repository::FilmRepository;
pub use postgres_film_repository::PostgresFilmRepository;
