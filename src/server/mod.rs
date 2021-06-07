use hyper::Server as HyperServer;
use routerify::RouterService;
use std::{
    str::FromStr,
    net::SocketAddr,
};
use tokio::sync::mpsc::Sender;
use tracing::info;
use anyhow::{Result, Context};
use crate::{
    db::DBMessage,
    config::CONFIG,
};

mod routes;

pub struct Server {
    db_sender: Sender<DBMessage>,
}

impl Server {
    pub fn new(db_sender: Sender<DBMessage>) -> Self {
        Self { db_sender }
    }

    pub async fn listen(&self) -> Result<()> {
        let router = routes::router()
            .data(self.db_sender.clone())
            .build()
            .unwrap();
        let service = RouterService::new(router).unwrap();
        let addr = SocketAddr::from_str(format!("{}:{}", CONFIG.host, CONFIG.port).as_str())
            .expect("Invalid host or port.");

        let server = HyperServer::bind(&addr).serve(service);
        info!("Server started listening on {}", addr);
        server.await.context("Unable to start server")?;

        Ok(())
    }
}
