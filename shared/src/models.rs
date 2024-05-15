use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[cfg_attr(feature = "backend", derive(FromRow))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct FilmModel {
    pub id: uuid::Uuid, // we will be using uuids as ids
    pub title: String,
    pub director: String,
    #[sqlx(try_from ="i16")]
    pub year: u16,      // only positive numbers
    pub poster: String, // we will use the url of the poster here
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[cfg_attr(feature = "backend", derive(FromRow))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct CreateFilmModel {
    pub title: String,
    pub director: String,
    #[sqlx(try_from ="i16")]
    pub year: u16,
    pub poster: String,
}
