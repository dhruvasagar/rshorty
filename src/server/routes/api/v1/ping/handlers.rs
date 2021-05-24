use hyper::{
    Body,
    Request,
    Response,
    http::Error,
};

pub async fn ping_get(_: Request<Body>) -> Result<Response<Body>, Error> {
    json!(body: "Hello Ping")
}

pub async fn pong_get(_: Request<Body>) -> Result<Response<Body>, Error> {
    json!(body: "Hello Pong")
}
