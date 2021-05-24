#[macro_export]
macro_rules! json {
    (body: $body:expr) => {
        hyper::Response::builder()
            .header(hyper::header::CONTENT_TYPE, "application/json")
            .body(serde_json::to_string($body).unwrap().into())
    };
    (status: $status:expr, body: $body:expr) => {
        hyper::Response::builder()
            .header(hyper::header::CONTENT_TYPE, "application/json")
            .status($status)
            .body(serde_json::to_string($body).unwrap().into())
    };
}

#[macro_export]
macro_rules! recv_dropped {
    ($m:expr, $f:tt) => {
        use tracing::error;
        match $m {
            Ok(_) => {}
            Err(_) => error!("Receiver was dropped for: {}", $f),
        }
    };
}
