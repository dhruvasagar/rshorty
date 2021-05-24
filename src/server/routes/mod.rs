use hyper::{
    Body,
    Request,
    Response,
    http::Error
};
use routerify::{
    Middleware,
    Router,
    RequestInfo,
    RouterBuilder
};
use routerify::ext::RequestExt;
use tracing::{error, info};

mod api;

pub fn router() -> RouterBuilder<Body, Error> {
    Router::builder()
        .middleware(Middleware::pre(logger_middleware))
        .get("/", home_get)
        .scope("/api", api::router())
        .err_handler_with_info(error_handler)
}

async fn logger_middleware(req: Request<Body>) -> Result<Request<Body>, Error> {
    info!(
        "{} {} {}",
        req.remote_addr(),
        req.method(),
        req.uri().path()
    );
    Ok(req)
}

async fn home_get(_: Request<Body>) -> Result<Response<Body>, Error> {
    json!(body: "Rshorty!")
}

async fn error_handler(err: routerify::RouteError, _: RequestInfo) -> Response<Body> {
    error!("{}", err);
    json!(status: hyper::StatusCode::INTERNAL_SERVER_ERROR, body: "Couldn't create a response while handling the server error")
        .unwrap()
}
