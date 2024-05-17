mod films;
mod health;

pub const API_VERSION: &str = "0.0.1";

pub use films::films_service;
pub use health::health_service;
