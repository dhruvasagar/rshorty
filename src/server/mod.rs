use hyper::Server as HyperServer;
use routerify::RouterService;
use std::{net::SocketAddr, str::FromStr};
use tokio::sync::mpsc::Sender;
use tracing::{error, info};

mod routes;
mod messages;
pub use messages::ServerMessage;

pub struct Server {
    message_sender: Sender<ServerMessage>,
}

impl Server {
    pub fn new(message_sender: Sender<ServerMessage>) -> Self {
        Self { message_sender }
    }

    pub async fn listen(&self, host: String, port: i64) {
        let router = routes::router()
            .data(self.message_sender.clone())
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
