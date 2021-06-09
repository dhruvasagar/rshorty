use serde::{Serialize, Deserialize};

#[derive(sqlx::FromRow, Debug, Clone, Serialize, Deserialize)]
pub struct UrlMapModel {
    pub key: String,
    pub url: String,
}

impl UrlMapModel {
    pub fn new(key: String, url: String) -> Self {
        Self { key, url }
    }
}
