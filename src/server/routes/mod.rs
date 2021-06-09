use hyper::{
    Body,
    Request,
    Response,
};
use routerify::{
    Router,
    Middleware,
    RequestInfo,
    RouterBuilder
};
use routerify::ext::RequestExt;
use tracing::{error, info};
use tokio::sync::mpsc::Sender;
use crate::{
    db::DBMessage,
    models::UrlMapModel,
};
use anyhow::{Error, Result};

mod api;

pub fn router() -> RouterBuilder<Body, Error> {
    Router::builder()
        .middleware(Middleware::pre(logger_middleware))
        .get("/", home_get)
        .get("/:key", redirect)
        .scope("/api", api::router())
        .err_handler_with_info(error_handler)
}

async fn logger_middleware(req: Request<Body>) -> Result<Request<Body>> {
    info!(
        "{} {} {}",
        req.remote_addr(),
        req.method(),
        req.uri().path()
    );
    Ok(req)
}

async fn home_get(_: Request<Body>) -> Result<Response<Body>> {
    Ok(Response::new(Body::from("rshorty!")))
}

async fn redirect(req: Request<Body>) -> Result<Response<Body>> {
    let (tx, rx) = tokio::sync::oneshot::channel();
    let sender = req.data::<Sender<DBMessage>>().unwrap();
    let key = req.param("key").unwrap().to_string();
    sender.send(DBMessage::GetUrlMap {
        key: key.clone(),
        resp: tx,
    }).await?;
    let url_map = rx.await.unwrap();
    let url_map = match url_map {
        Ok(u) => u,
        Err(_) => {
            UrlMapModel::new(key, "/".to_string())
        }
    };
    match Response::builder()
        .header(hyper::header::LOCATION, url_map.url)
        .status(hyper::StatusCode::SEE_OTHER)
        .body(Body::from("redirecting..."))
        {
            Ok(x) => Ok(x),
            Err(e) => Err(anyhow::anyhow!(e.to_string())),
        }
}

async fn error_handler(err: routerify::RouteError, _: RequestInfo) -> Response<Body> {
    error!("{}", err);
    json!(status: hyper::StatusCode::INTERNAL_SERVER_ERROR, body: "Couldn't create a response while handling the server error")
        .unwrap()
}
