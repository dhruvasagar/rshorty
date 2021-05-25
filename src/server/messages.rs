use crate::models::UrlMapModel;
use crate::types::OneShotMessageResponse;

pub enum ServerMessage {
    GetUrlMaps {
        offset: Option<i64>,
        resp: OneShotMessageResponse<Result<Vec<UrlMapModel>, sqlx::Error>>,
    },
    GetUrlMap {
        key: String,
        resp: OneShotMessageResponse<Result<UrlMapModel, sqlx::Error>>,
    },
    CreateUrlMap {
        key: String,
        url: String,
        resp: OneShotMessageResponse<Result<UrlMapModel, sqlx::Error>>,
    },
    UpdateUrlMap {
        key: String,
        url: String,
        resp: OneShotMessageResponse<Result<UrlMapModel, sqlx::Error>>,
    },
    DeleteUrlMap {
        key: String,
        resp: OneShotMessageResponse<Result<UrlMapModel, sqlx::Error>>,
    },
}

// impl ServerMessage {
//     pub fn get_type<'a>(&'a self) -> &'a str {
//         return match self {
//             ServerMessage::GetUrlMap { .. } => "GetUrlMap",
//             ServerMessage::GetUrlMaps { .. } => "GetUrlMaps",
//             ServerMessage::CreateUrlMap { .. } => "CreateUrlMap",
//             ServerMessage::UpdateUrlMap { .. } => "UpdateUrlMap",
//             ServerMessage::DeleteUrlMap { .. } => "DeleteUrlMap",
//         };
//     }
// }
