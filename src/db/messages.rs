use crate::models::UrlMapModel;

#[derive(Debug)]
pub enum DBMessage {
    GetUrlMap {
        key: String,
        resp: DBMessageResponse<UrlMapModel>,
    },
    GetUrlMaps {
        offset: Option<i64>,
        resp: DBMessageResponse<Vec<UrlMapModel>>,
    },
    CreateUrlMap {
        url_map: UrlMapModel,
        resp: DBMessageResponse<UrlMapModel>,
    },
    UpdateUrlMap {
        url_map: UrlMapModel,
        resp: DBMessageResponse<UrlMapModel>,
    },
    DeleteUrlMap {
        key: String,
        resp: DBMessageResponse<UrlMapModel>,
    },
}

impl DBMessage {
    pub fn get_type<'a>(&'a self) -> &'a str {
        match self {
            DBMessage::GetUrlMap { .. } => "GetUrlMap",
            DBMessage::GetUrlMaps { .. } => "GetUrlMaps",
            DBMessage::CreateUrlMap { .. } => "CreateUrlMap",
            DBMessage::UpdateUrlMap { .. } => "UpdateUrlMap",
            DBMessage::DeleteUrlMap { .. } => "DeleteUrlMap",
        }
    }
}

pub type DBMessageResponse<T> = tokio::sync::oneshot::Sender<Result<T, sqlx::Error>>;
