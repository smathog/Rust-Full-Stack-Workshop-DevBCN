use async_trait::async_trait;
use shared::models::{CreateFilmModel, FilmModel};
use uuid::Uuid;

pub type FilmError = String;
pub type FilmResult<T> = Result<T, FilmError>;

#[async_trait]
pub trait FilmRepository: Send + Sync + 'static {
    async fn get_films(&self) -> FilmResult<Vec<FilmModel>>;
    async fn get_film(&self, id: &Uuid) -> FilmResult<FilmModel>;
    async fn create_film(&mut self, model: &CreateFilmModel) -> FilmResult<FilmModel>;
    async fn update_film(&mut self, model: &FilmModel) -> FilmResult<FilmModel>;
    async fn delete_film(&mut self, id: &Uuid) -> FilmResult<Uuid>;
}
