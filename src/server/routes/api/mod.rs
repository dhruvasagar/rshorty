use hyper::Body;
use anyhow::Error;
use routerify::Router;

mod v1;

pub fn router() -> Router<Body, Error> {
    Router::builder()
        .scope("/v1", v1::router())
        .build()
        .unwrap()
}
