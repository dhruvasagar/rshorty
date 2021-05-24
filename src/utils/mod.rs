use hyper::{header::HeaderName, Body, Request};
use routerify::ext::RequestExt;
use std::net::IpAddr;

pub fn extract_client_ip_from_req(req: &Request<Body>) -> IpAddr {
    req.headers()
        .get(HeaderName::from_static("x-forwarded-for"))
        .and_then(|v| v.to_str().ok())
        .and_then(|ips| ips.split(",").nth(0))
        .map(|ip| ip.trim())
        .and_then(|ip| ip.parse::<IpAddr>().ok())
        .unwrap_or(req.remote_addr().ip())
}
