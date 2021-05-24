use sqlx::{pool::PoolConnection, Pool, Sqlite};
use tokio::sync::mpsc::Receiver;
use crate::models::UrlMapModel;
use super::DBMessage;
use tracing::{error, info};

pub struct DBManager {
    pool: Pool<Sqlite>,
    rx: Receiver<DBMessage>,
}

type Connection = PoolConnection<Sqlite>;

impl DBManager {
    pub fn new(pool: Pool<Sqlite>, rx: Receiver<DBMessage>) -> Self {
        Self { pool, rx }
    }

    pub async fn create_url_map(
        conn: &mut Connection,
        url_map: UrlMapModel,
    ) -> Result<UrlMapModel, sqlx::Error> {
        let row = sqlx::query_as!(UrlMapModel, r#"
                INSERT INTO url_maps (key, url)
                VALUES ($1, $2)
            "#,
            url_map.key,
            url_map.url
            )
            .fetch_one(conn).await;
        return row;
    }

    pub async fn get_url_map(
        conn: &mut Connection,
        key: String,
    ) -> Result<UrlMapModel, sqlx::Error> {
        let url_map = sqlx::query_as!(UrlMapModel, "SELECT * FROM url_maps WHERE id = $1", key)
            .fetch_one(conn)
            .await;
        return url_map;
    }

    pub async fn get_url_maps(
        conn: &mut Connection,
        offset: Option<i64>,
    ) -> Result<Vec<UrlMapModel>, sqlx::Error> {
        let offset = offset.unwrap_or(0);
        let url_maps = sqlx::query_as!(UrlMapModel, r#"
            SELECT * FROM url_maps
            LIMIT 100
            OFFSET $1
            "#,
            offset
            )
            .fetch_all(conn)
            .await;
        return url_maps;
    }

    pub async fn update_url_map(
        conn: &mut Connection,
        url_map: UrlMapModel,
    ) -> Result<UrlMapModel, sqlx::Error> {
        let row = sqlx::query_as!(UrlMapModel, r#"
            UPDATE url_maps
            SET url = $2
            WHERE key = $1
            "#,
            url_map.key,
            url_map.url
            )
            .fetch_one(conn)
            .await;
        row
    }

    pub async fn delete_url_map(
        conn: &mut Connection,
        key: String,
    ) -> Result<UrlMapModel, sqlx::Error> {
        let row = sqlx::query_as!(
            UrlMapModel,
            "DELETE FROM url_maps where key = $1",
            key
            )
            .fetch_one(conn)
            .await;
        row
    }

    pub async fn listen(&mut self) {
        info!("DBManager started listening for messages.");
        while let Some(message) = self.rx.recv().await {
            info!("Got a {} message", message.get_type());
            let mut connection = self.pool.acquire().await.unwrap();
            let pool = self.pool.clone();
            tokio::spawn(async move {
                match message {
                    DBMessage::GetUrlMaps { offset, resp } => {
                        let url_maps = Self::get_url_maps(&mut connection, offset).await;
                        recv_dropped!(resp.send(url_maps), "GetUrlMaps");
                    }
                    DBMessage::GetUrlMap { key, resp } => {
                        let url_map = Self::get_url_map(&mut connection, key).await;
                        recv_dropped!(resp.send(url_map), "GetUrlMap");
                    }
                    DBMessage::CreateUrlMap { url_map, resp } => {
                        let url_map = Self::create_url_map(&mut connection, url_map).await;
                        recv_dropped!(resp.send(url_map), "CreateUrlMap");
                    }
                    DBMessage::UpdateUrlMap { url_map, resp } => {
                        let url_map = Self::update_url_map(&mut connection, url_map).await;
                        recv_dropped!(resp.send(url_map), "UpdateUrlMap");
                    }
                    DBMessage::DeleteUrlMap { key, resp } => {
                        let url_map = Self::delete_url_map(&mut connection, key).await;
                        recv_dropped!(resp.send(url_map), "DeleteUrlMap");
                    }
                }
            });
        }
    }
}
