mod manager;
mod messages;
pub use manager::DBManager;
pub use messages::DBMessage;
use sqlx::{Pool, Sqlite, sqlite::SqlitePoolOptions, migrate};

pub async fn connect<'a>(db_url: &'a str) -> Result<Pool<Sqlite>, sqlx::Error> {
    let pool = SqlitePoolOptions::new()
        .connect(db_url)
        .await?;

    let migrator = migrate!();
    match migrator.run(&pool).await {
        Ok(_) => {},
        Err(e) => {
            panic!("Couldn't migrate!, {}", e.to_string());
        }
    };
    Ok(pool)
}
