use crate::{config::CONFIG, db::DBMessage};
use anyhow::{Context, Result};
use hyper::Server as HyperServer;
use routerify::RouterService;
use std::{net::SocketAddr, str::FromStr};
use tokio::sync::mpsc::Sender;
use tracing::info;

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
        let addr = SocketAddr::from_str(&format!("{}:{}", CONFIG.host, CONFIG.port))
            .expect("Invalid host or port.");

        let server = HyperServer::bind(&addr).serve(service);
        info!("Server started listening on {}", addr);
        server.await.context("Unable to start server")?;

        Ok(())
    }
}
