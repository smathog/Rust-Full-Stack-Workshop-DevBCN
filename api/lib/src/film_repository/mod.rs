mod film_repository;
mod memory_film_repository;
mod postgres_film_repository;

pub use film_repository::*;
pub use memory_film_repository::*;
pub use postgres_film_repository::*;
