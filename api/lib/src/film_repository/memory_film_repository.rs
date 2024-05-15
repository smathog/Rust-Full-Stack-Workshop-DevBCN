use crate::{FilmRepository, FilmResult};
use async_trait::async_trait;
use shared::models::{CreateFilmModel, FilmModel};
use sqlx::types::chrono;
use std::collections::HashMap;
use uuid::Uuid;

pub struct MemoryFilmRepository {
    map: HashMap<Uuid, FilmModel>,
}

impl MemoryFilmRepository {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
}

#[async_trait]
impl FilmRepository for MemoryFilmRepository {
    async fn get_films(&self) -> FilmResult<Vec<FilmModel>> {
        self.map.values().map(|film| Ok(film.clone())).collect()
    }

    async fn get_film(&self, id: &Uuid) -> FilmResult<FilmModel> {
        self.map
            .get(id)
            .map(|film| film.clone())
            .ok_or(format!("No Film with id {} found.", id))
    }

    async fn create_film(&mut self, model: &CreateFilmModel) -> FilmResult<FilmModel> {
        let CreateFilmModel {
            title,
            director,
            year,
            poster,
        } = model.clone();
        let id = Uuid::new_v4();
        let film = FilmModel {
            id,
            title: title.clone(),
            director: director.clone(),
            year,
            poster: poster.clone(),
            created_at: Some(chrono::Utc::now()),
            updated_at: Some(chrono::Utc::now()),
        };
        // id technically might not be unique, but this is unlikely to the point it need not be
        // considered
        self.map.insert(id, film.clone());
        Ok(film)
    }

    async fn update_film(&mut self, model: &FilmModel) -> FilmResult<FilmModel> {
        let entry = self
            .map
            .get_mut(&model.id)
            .ok_or(format!("No Film with id {} found.", &model.id))?;
        *entry = model.clone();
        Ok(model.clone())
    }

    async fn delete_film(&mut self, id: &Uuid) -> FilmResult<Uuid> {
        self.map
            .remove(id)
            .ok_or(format!("No Film with id {} found.", id))
            .map(|_| *id)
    }
}
