use hyper::{
    Body,
    Request,
    Response,
};
use anyhow::Result;

pub async fn ping_get(_: Request<Body>) -> Result<Response<Body>> {
    json!(body: "Hello Ping")
}

pub async fn pong_get(_: Request<Body>) -> Result<Response<Body>> {
    json!(body: "Hello Pong")
}
