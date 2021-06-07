mod manager;
mod messages;
pub use manager::DBManager;
pub use messages::DBMessage;
use sqlx::{Pool, Sqlite, sqlite::SqlitePoolOptions, migrate};
use anyhow::{Result, Context};
use crate::config::CONFIG;

pub struct DB {
    pub pool: Pool<Sqlite>,
}

impl DB {
    pub async fn new() -> Result<Self> {
        let pool = SqlitePoolOptions::new()
            .connect(&CONFIG.database.url)
            .await
            .context("Unable to connect to database")?;

        let migrator = migrate!();
        migrator
            .run(&pool)
            .await
            .context("Unable to run migrations")?;
        Ok(Self{pool})
    }
}
