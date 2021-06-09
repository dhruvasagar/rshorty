use hyper::{
    Body,
    Request,
    Response,
    body::to_bytes
};
use routerify::ext::RequestExt;
use tokio::sync::mpsc::Sender;
use serde::{Serialize, Deserialize};
use crate::{
    db::DBMessage,
    models::UrlMapModel,
};
use anyhow::{Result, Context};

pub async fn get_url_maps(req: Request<Body>) -> Result<Response<Body>> {
    let (tx, rx) = tokio::sync::oneshot::channel();
    let sender = req.data::<Sender<DBMessage>>().unwrap();
    sender.send(DBMessage::GetUrlMaps {
        offset: None,
        resp: tx,
    }).await?;
    match_result!(rx.await.unwrap())
}

pub async fn get_url_map(req: Request<Body>) -> Result<Response<Body>> {
    let (tx, rx) = tokio::sync::oneshot::channel();
    let sender = req.data::<Sender<DBMessage>>().unwrap();
    let key = req.param("key").unwrap().to_string();
    sender.send(DBMessage::GetUrlMap {
        key,
        resp: tx,
    }).await?;
    match_result!(rx.await.unwrap())
}

pub async fn create_url_map(mut req: Request<Body>) -> Result<Response<Body>> {
    let (tx, rx) = tokio::sync::oneshot::channel();
    let body = req.body_mut();
    let url_map_bytes = to_bytes(body).await?;
    let url_map = serde_json::from_slice::<UrlMapModel>(&url_map_bytes)
        .context("Unable to parse request body")?;
    let sender = req.data::<Sender<DBMessage>>().unwrap();
    sender.send(DBMessage::CreateUrlMap {
        url_map,
        resp: tx,
    }).await?;
    match_result!(rx.await.unwrap())
}

pub async fn update_url_map(mut req: Request<Body>) -> Result<Response<Body>> {
    #[derive(Debug, Serialize, Deserialize)]
    struct Url {
        url: String,
    }
    let key = req.param("key").unwrap().to_string();
    let body = req.body_mut();
    let url_map_bytes = to_bytes(body).await?;
    let url = serde_json::from_slice::<Url>(&url_map_bytes)
        .context("Unable to parse request body")?;
    let url_map = UrlMapModel::new(key, url.url);
    let (tx, rx) = tokio::sync::oneshot::channel();
    let sender = req.data::<Sender<DBMessage>>().unwrap();
    sender.send(DBMessage::UpdateUrlMap {
        url_map,
        resp: tx,
    }).await?;
    match_result!(rx.await.unwrap())
}

pub async fn delete_url_map(req: Request<Body>) -> Result<Response<Body>> {
    let (tx, rx) = tokio::sync::oneshot::channel();
    let sender = req.data::<Sender<DBMessage>>().unwrap();
    let key = req.param("key").unwrap().to_string();
    sender.send(DBMessage::DeleteUrlMap {
        key,
        resp: tx,
    }).await?;
    match rx.await.unwrap() {
        Ok(_) => {
            let obj = serde_json::json!({
                "ok": "true"
            }).to_string();
            json!(status: hyper::StatusCode::OK, body: &obj)
        }
        Err(e) => {
            let obj = serde_json::json!({
                "error": e.to_string()
            }).to_string();
            json!(status: hyper::StatusCode::NOT_FOUND, body: &obj)
        }
    }
}
