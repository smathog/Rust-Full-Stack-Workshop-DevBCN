use actix_web::web;
use tokio::sync::RwLock;
use crate::FilmRepository;

mod films_static;
mod health;
mod films_dynamic;

pub const API_VERSION: &str = "0.0.1";

pub type RepoTypeDynamic = web::Data<RwLock<Box<dyn FilmRepository>>>;

pub use films_static::*;
pub use films_dynamic::*;
pub use health::*;
