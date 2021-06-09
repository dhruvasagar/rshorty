#[macro_export]
macro_rules! json {
    (body: $body:expr) => {
        match hyper::Response::builder()
            .header(hyper::header::CONTENT_TYPE, "application/json")
            .body(serde_json::to_string($body).unwrap().into())
        {
            Ok(x) => Ok(x),
            Err(e) => Err(anyhow::anyhow!(e.to_string())),
        }
    };
    (status: $status:expr, body: $body:expr) => {
        match hyper::Response::builder()
            .header(hyper::header::CONTENT_TYPE, "application/json")
            .status($status)
            .body(serde_json::to_string($body).unwrap().into())
        {
            Ok(x) => Ok(x),
            Err(e) => Err(anyhow::anyhow!(e.to_string())),
        }
    };
}

#[macro_export]
macro_rules! match_result {
    ($res:expr) => {
        match $res {
            Ok(result) => json!(body: &result),
            Err(e) => {
                let obj = serde_json::json!({
                    "error": e.to_string(),
                }).to_string();
                json!(status: hyper::StatusCode::INTERNAL_SERVER_ERROR, body: &obj)
            }
        }
    }
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

#[macro_export]
macro_rules! empty_malformed_body {
    () => {
        return {
            let obj = serde_json::json!({
                "error": "Body is malformed/empty, please try again."
            }).to_string();
            json!(status: hyper::StatusCode::BAD_REQUEST, body: &obj)
        };
    }
}
