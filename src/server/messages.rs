use crate::models::UrlMapModel;
use tokio::sync::oneshot::Sender;

pub enum ServerMessage {
    GetUrlMaps {
        offset: Option<i64>,
        resp: Sender<Result<Vec<UrlMapModel>, sqlx::Error>>,
    },
    GetUrlMap {
        key: String,
        resp: Sender<Result<UrlMapModel, sqlx::Error>>,
    },
    CreateUrlMap {
        key: String,
        url: String,
        resp: Sender<Result<UrlMapModel, sqlx::Error>>,
    },
    UpdateUrlMap {
        key: String,
        url: String,
        resp: Sender<Result<UrlMapModel, sqlx::Error>>,
    },
    DeleteUrlMap {
        key: String,
        resp: Sender<Result<UrlMapModel, sqlx::Error>>,
    },
}

impl ServerMessage {
    pub fn get_type<'a>(&'a self) -> &'a str {
        return match self {
            ServerMessage::GetUrlMap { .. } => "GetUrlMap",
            ServerMessage::GetUrlMaps { .. } => "GetUrlMaps",
            ServerMessage::CreateUrlMap { .. } => "CreateUrlMap",
            ServerMessage::UpdateUrlMap { .. } => "UpdateUrlMap",
            ServerMessage::DeleteUrlMap { .. } => "DeleteUrlMap",
        };
    }
}
