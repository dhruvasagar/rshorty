use hyper::Body;
use routerify::{Error, Router};
pub mod handlers;

pub fn router() -> Router<Body, Error> {
    Router::builder()
        .get("/", handlers::get_url_maps)
        .get("/:key", handlers::get_url_map)
        .post("/", handlers::create_url_map)
        .put("/:key", handlers::update_url_map)
        .delete("/:key", handlers::delete_url_map)
        .build()
        .unwrap()
}
