use std::error::Error;
use derive_more::{Display};

#[derive(Debug, Display)]
pub struct SqlxErrorWrapper(pub sqlx::Error);

impl Error for SqlxErrorWrapper {}

