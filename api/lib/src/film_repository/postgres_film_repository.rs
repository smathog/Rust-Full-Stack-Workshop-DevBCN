use crate::{FilmRepository, FilmResult};
use async_trait::async_trait;
use shared::models::{CreateFilmModel, FilmModel};
use sqlx::{query_as, query_scalar, PgPool};
use uuid::Uuid;

pub struct PostgresFilmRepository {
    pool: PgPool,
}

impl PostgresFilmRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl FilmRepository for PostgresFilmRepository {
    async fn get_films(&self) -> FilmResult<Vec<FilmModel>> {
        query_as::<_, FilmModel>(
            r#"
            SELECT id, title, director, year, poster, created_at, updated_at
            FROM films
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn get_film(&self, id: &Uuid) -> FilmResult<FilmModel> {
        query_as::<_, FilmModel>(
            r#"
            SELECT id, title, director, year, poster, created_at, updated_at
            FROM films
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn create_film(&mut self, model: &CreateFilmModel) -> FilmResult<FilmModel> {
        query_as::<_, FilmModel>(
            r#"
            INSERT INTO films (title, director, year, poster)
            VALUES ($1, $2, $3, $4)
            RETURNING id, title, director, year, poster, created_at, updated_at
            "#,
        )
        .bind(&model.title)
        .bind(&model.director)
        .bind(model.year as i16)
        .bind(&model.poster)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn update_film(&mut self, model: &FilmModel) -> FilmResult<FilmModel> {
        query_as::<_, FilmModel>(
            r#"
            UPDATE films
            SET title = $2, directory = $3, year = $4, poster = $5
            WHERE id = $1
            RETURNING id, title, director, year, poster, created_at, updated_at
            "#,
        )
        .bind(model.id)
        .bind(&model.title)
        .bind(&model.director)
        .bind(model.year as i16)
        .bind(&model.poster)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn delete_film(&mut self, id: &Uuid) -> FilmResult<Uuid> {
        query_scalar::<_, Uuid>(
            r#"
            DELETE FROM films
            WHERE id = $1
            RETURNING id
            "#,
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }
}
