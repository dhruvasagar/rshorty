use hyper::{http::Error, Body};
use routerify::Router;

mod ping;

pub fn router() -> Router<Body, Error> {
    Router::builder()
        .get("/ping", ping::handlers::ping_get)
        .get("/pong", ping::handlers::pong_get)
        .build()
        .unwrap()
}
