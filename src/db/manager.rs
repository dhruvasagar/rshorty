use sqlx::{pool::PoolConnection, Sqlite};
use tokio::sync::mpsc::Receiver;
use crate::{db::DB, models::UrlMapModel};
use super::DBMessage;
use tracing::{error, info};

pub struct DBManager {
    db: DB,
    rx: Receiver<DBMessage>,
}

type Connection = PoolConnection<Sqlite>;

impl DBManager {
    pub fn new(db: DB, rx: Receiver<DBMessage>) -> Self {
        Self { db, rx }
    }

    pub async fn create_url_map(
        conn: &mut Connection,
        url_map: UrlMapModel,
    ) -> Result<UrlMapModel, sqlx::Error> {
        let row = sqlx::query_as::<_, UrlMapModel>("INSERT INTO url_maps (key, url) VALUES (?, ?) RETURNING *")
            .bind(url_map.key)
            .bind(url_map.url)
            .fetch_one(conn).await;
        return row;
    }

    pub async fn get_url_map(
        conn: &mut Connection,
        key: String,
    ) -> Result<UrlMapModel, sqlx::Error> {
        let url_map = sqlx::query_as::<_, UrlMapModel>("SELECT * FROM url_maps WHERE key = ?")
            .bind(key)
            .fetch_one(conn)
            .await;
        return url_map;
    }

    pub async fn get_url_maps(
        conn: &mut Connection,
        offset: Option<i64>,
    ) -> Result<Vec<UrlMapModel>, sqlx::Error> {
        let offset = offset.unwrap_or(0);
        let url_maps = sqlx::query_as::<_, UrlMapModel>("SELECT * FROM url_maps LIMIT 100 OFFSET ?")
            .bind(offset)
            .fetch_all(conn)
            .await;
        return url_maps;
    }

    pub async fn update_url_map(
        conn: &mut Connection,
        url_map: UrlMapModel,
    ) -> Result<UrlMapModel, sqlx::Error> {
        let row = sqlx::query_as::<_, UrlMapModel>("UPDATE url_maps SET url = ? WHERE key = ? RETURNING *")
            .bind(url_map.url)
            .bind(url_map.key)
            .fetch_one(conn)
            .await;
        row
    }

    pub async fn delete_url_map(
        conn: &mut Connection,
        key: String,
    ) -> Result<UrlMapModel, sqlx::Error> {
        let row = sqlx::query_as::<_, UrlMapModel>("DELETE FROM url_maps where key = ? RETURNING *")
            .bind(key)
            .fetch_one(conn)
            .await;
        row
    }

    pub async fn listen(&mut self) {
        info!("DBManager started listening for messages.");
        while let Some(message) = self.rx.recv().await {
            info!("Got a {} message", message.get_type());
            let pool = self.db.pool.clone();
            let mut connection = pool.acquire().await.unwrap();
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
