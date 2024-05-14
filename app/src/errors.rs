use derive_more::Display;
use std::error::Error;

#[derive(Debug, Display)]
pub struct SqlxErrorWrapper(pub sqlx::Error);

impl Error for SqlxErrorWrapper {}
