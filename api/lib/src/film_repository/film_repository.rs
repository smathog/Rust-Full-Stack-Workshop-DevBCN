use async_trait::async_trait;
use shared::models::{CreateFilmModel, FilmModel};
use std::ops::Deref;
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

// Allow dynamic and static dispatch from the same generic API implementation
#[async_trait]
impl<T> FilmRepository for T
where
    T: Deref<Target = dyn FilmRepository> + Sync + Send + 'static,
{
    async fn get_films(&self) -> FilmResult<Vec<FilmModel>> {
        self.get_films().await
    }

    async fn get_film(&self, id: &Uuid) -> FilmResult<FilmModel> {
        self.get_film(id).await
    }

    async fn create_film(&mut self, model: &CreateFilmModel) -> FilmResult<FilmModel> {
        self.create_film(model).await
    }

    async fn update_film(&mut self, model: &FilmModel) -> FilmResult<FilmModel> {
        self.update_film(model).await
    }

    async fn delete_film(&mut self, id: &Uuid) -> FilmResult<Uuid> {
        self.delete_film(id).await
    }
}
