use hyper::Body;
use routerify::{Error, Router};

mod ping;
mod url_maps;

pub fn router() -> Router<Body, Error> {
    Router::builder()
        .get("/ping", ping::handlers::ping_get)
        .get("/pong", ping::handlers::pong_get)
        .scope("/url_maps", url_maps::router())
        .build()
        .unwrap()
}
