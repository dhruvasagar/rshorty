use hyper::{
    Body,
    Request,
    Response,
    body::HttpBody,
};
use routerify::{
    Error,
    ext::RequestExt,
};
use serde::{Serialize, Deserialize};
use tokio::sync::mpsc::Sender;
use crate::{
    db::DBMessage,
    models::UrlMapModel,
};
use tracing::error;

pub async fn get_url_maps(req: Request<Body>) -> Result<Response<Body>, Error> {
    let (tx, rx) = tokio::sync::oneshot::channel();
    let sender = req.data::<Sender<DBMessage>>().unwrap();
    sender_failed!(
        sender
        .send(DBMessage::GetUrlMaps {
            offset: None,
            resp: tx,
        })
        .await, "GetUrlMaps");
    match_result!(rx.await.unwrap())
}

pub async fn get_url_map(req: Request<Body>) -> Result<Response<Body>, Error> {
    let (tx, rx) = tokio::sync::oneshot::channel();
    let sender = req.data::<Sender<DBMessage>>().unwrap();
    let key = req.param("key").unwrap().to_string();
    sender_failed!(
        sender
        .send(DBMessage::GetUrlMap {
            key,
            resp: tx,
        })
        .await, "GetUrlMap");
    match_result!(rx.await.unwrap())
}

pub async fn create_url_map(mut req: Request<Body>) -> Result<Response<Body>, Error> {
    #[derive(Debug, Serialize, Deserialize)]
    struct RequestBody {
        key: String,
        url: String,
    }
    let (tx, rx) = tokio::sync::oneshot::channel();
    let body = req.body_mut();
    if let Some(Ok(body)) = body.data().await {
        if let Ok(json_value) =
            serde_json::from_slice(&body) as Result<RequestBody, serde_json::Error>
            {
                let sender = req.data::<Sender<DBMessage>>().unwrap();
                let url_map = UrlMapModel::new(json_value.key, json_value.url);
                sender_failed!(
                    sender
                    .send(DBMessage::CreateUrlMap {
                        url_map,
                        resp: tx,
                    })
                    .await, "CreateUrlMap");
                return match_result!(rx.await.unwrap());
            } else {
                println!("{:?}", &body);
            }
    }
    empty_malformed_body!()
}

pub async fn update_url_map(mut req: Request<Body>) -> Result<Response<Body>, Error> {
    let (utx, urx) = tokio::sync::oneshot::channel();
    let key = req.param("key").unwrap().to_string();
    let sender = req.data::<Sender<DBMessage>>().unwrap();
    sender_failed!(
        sender
        .send(DBMessage::GetUrlMap {
            key,
            resp: utx,
        })
        .await, "GetUrlMap");
    let url_map = urx.await.unwrap();
    let mut url_map = match url_map {
        Ok(t) => t,
        Err(e) => {
            let obj = serde_json::json!({
                "error": e.to_string(),
            }).to_string();
            return json!(status: hyper::StatusCode::INTERNAL_SERVER_ERROR, body: &obj);
        }
    };

    #[derive(Debug, Serialize, Deserialize)]
    struct RequestBody {
        url: String,
    }
    let body = req.body_mut();
    if let Some(Ok(body)) = body.data().await {
        if let Ok(json_value) =
            serde_json::from_slice(&body) as Result<RequestBody, serde_json::Error>
            {
                let sender = req.data::<Sender<DBMessage>>().unwrap();
                url_map.url = json_value.url;
                let (tx, rx) = tokio::sync::oneshot::channel();
                sender_failed!(
                    sender
                    .send(DBMessage::UpdateUrlMap {
                        url_map,
                        resp: tx,
                    })
                    .await, "UpdateUrlMap");
                return match_result!(rx.await.unwrap());
            } else {
                println!("{:?}", &body);
            }
    }
    empty_malformed_body!()
}

pub async fn delete_url_map(req: Request<Body>) -> Result<Response<Body>, Error> {
    let (tx, rx) = tokio::sync::oneshot::channel();
    let sender = req.data::<Sender<DBMessage>>().unwrap();
    let key = req.param("key").unwrap().to_string();
    sender_failed!(
        sender
        .send(DBMessage::DeleteUrlMap {
            key,
            resp: tx,
        })
        .await, "DeleteUrlMap");
    return match_result!(rx.await.unwrap());
}
