use hyper::Server as HyperServer;
use routerify::RouterService;
use std::{
    str::FromStr,
    net::SocketAddr,
};
use tokio::sync::mpsc::Sender;
use tracing::{error, info};
use crate::db::DBMessage;

mod routes;

pub struct Server {
    db_sender: Sender<DBMessage>,
}

impl Server {
    pub fn new(db_sender: Sender<DBMessage>) -> Self {
        Self { db_sender }
    }

    pub async fn listen(&self, host: String, port: i64) {
        let router = routes::router()
            .data(self.db_sender.clone())
            .build()
            .unwrap();
        let service = RouterService::new(router).unwrap();
        let addr = SocketAddr::from_str(format!("{}:{}", host, port).as_str())
            .expect("Invalid host or port.");
        let server = HyperServer::bind(&addr).serve(service);
        info!("Server started listening on {}", addr);
        if let Err(err) = server.await {
            error!("Server error: {}", err);
        }
    }
}
