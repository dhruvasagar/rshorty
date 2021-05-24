use serde::Serialize;

#[derive(sqlx::FromRow, Debug, Clone, Serialize)]
pub struct UrlMapModel {
    pub key: String,
    pub url: String,
}

impl UrlMapModel {
    pub fn new(key: String, url: String) -> Self {
        let mut url_map = Self { key, url };
        return url_map;
    }
}
