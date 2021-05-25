use crate::models::UrlMapModel;

pub enum ReactorMessage {
    ServerGetUrlMap {
        key: String,
        resp: ComposedResponse<UrlMapModel>,
    },
    ServerGetUrlMaps {
        offset: Option<i64>,
        resp: ComposedResponse<Vec<UrlMapModel>>,
    },
    ServerCreateUrlMap {
        key: String,
        url: String,
        resp: ComposedResponse<UrlMapModel>,
    },
    ServerUpdateUrlMap {
        key: String,
        url: String,
        resp: ComposedResponse<UrlMapModel>,
    },
    ServerDeleteUrlMap {
        key: String,
        resp: ComposedResponse<UrlMapModel>,
    },
}

type ComposedResponse<T> = tokio::sync::oneshot::Sender<Result<T, sqlx::Error>>;

impl ReactorMessage {
    pub fn get_type<'a>(&'a self) -> &'a str {
        return match self {
            ReactorMessage::ServerGetUrlMap { .. } => "ServerGetUrlMap",
            ReactorMessage::ServerGetUrlMaps { .. } => "ServerGetUrlMaps",
            ReactorMessage::ServerCreateUrlMap { .. } => "ServerCreateUrlMap",
            ReactorMessage::ServerUpdateUrlMap { .. } => "ServerUpdateUrlMap",
            ReactorMessage::ServerDeleteUrlMap { .. } => "ServerDeleteUrlMap",
        };
    }
}
