use tokio::sync::{oneshot, Mutex};
use crate::{
    db::DBMessage,
    models::UrlMapModel,
    server::ServerMessage,
    types::{DBSender, ReactorReceiver, ReactorSender, ServerReceiver},
};
pub use messages::ReactorMessage;
use std::sync::Arc;
use tracing::{error, info};

mod messages;

pub struct Reactor {
    pub db_sender: DBSender,
    pub server_receiver: Arc<Mutex<ServerReceiver>>,
    pub inner_sender: ReactorSender,
}

impl Reactor {
    pub async fn listen_for_server(receiver: &mut ServerReceiver, inner_sender: ReactorSender) {
        while let Some(message) = receiver.recv().await {
            let reactor_message = match message {
                ServerMessage::GetUrlMaps { offset, resp } => {
                    ReactorMessage::ServerGetUrlMaps { offset, resp }
                }
                ServerMessage::GetUrlMap { key, resp } => {
                    ReactorMessage::ServerGetUrlMap { key, resp }
                }
                ServerMessage::CreateUrlMap { key, url, resp } => {
                    ReactorMessage::ServerCreateUrlMap { key, url, resp }
                }
                ServerMessage::UpdateUrlMap { key, url, resp } => {
                    ReactorMessage::ServerUpdateUrlMap { key, url, resp }
                }
                ServerMessage::DeleteUrlMap { key, resp } => {
                    ReactorMessage::ServerDeleteUrlMap { key, resp }
                }
            };
            inner_sender.send(reactor_message).await.unwrap_or_default();
        }
    }

    pub async fn listen(&mut self, mut receiver: ReactorReceiver) {
        let inner_sender = self.inner_sender.clone();
        let server_receiver = self.server_receiver.clone();
        tokio::spawn(async move {
            let mut server_receiver = server_receiver.lock().await;
            let server_receiver = &mut *server_receiver;
            Self::listen_for_server(server_receiver, inner_sender).await;
        });
        while let Some(message) = receiver.recv().await {
            macro_rules! didnt_receive {
                ($send: expr, $sender_type: expr, $msg: tt) => {
                    match $send {
                        Ok(_) => {},
                        Err(e) => {
                            error!("{} didnt receive the {} message!", $sender_type, $msg);
                            panic!("{}", e.to_string());
                        }
                    }
                }
            }
            macro_rules! server_receiver_dropped {
                ($resp: expr, $msg: tt) => {
                    $resp.expect(&format!("Server receiver is dropped, {}", $msg));
                }
            }
            let msg_type = message.get_type().to_string();
            info!("Reactor got message {}", msg_type);
            let db_sender = self.db_sender.clone();
            let inner_sender = self.inner_sender.clone();
            tokio::spawn(async move {
                match message {
                    ReactorMessage::ServerGetUrlMap { key, resp } => {
                        let (tx, rx) = oneshot::channel();
                        didnt_receive!(
                            db_sender
                            .send(DBMessage::GetUrlMap {
                                key: key, resp: tx
                            })
                            .await, "Database", "GetUrlMap");
                        let result = rx.await.unwrap();
                        server_receiver_dropped!(resp.send(result), "ServerGetUrlMap");
                    }
                    ReactorMessage::ServerGetUrlMaps { offset, resp } => {
                        let (tx, rx) = oneshot::channel();
                        didnt_receive!(
                            db_sender
                            .send(DBMessage::GetUrlMaps {
                                offset,
                                resp: tx,
                            })
                            .await, "Database", "GetUrlMaps");
                        let result = rx.await.unwrap();
                        server_receiver_dropped!(resp.send(result), "ServerGetUrlMaps");
                    }
                    ReactorMessage::ServerCreateUrlMap { key, url, resp } => {
                        let url_map = UrlMapModel::new(key, url);
                        let (tx, rx) = oneshot::channel();
                        didnt_receive!(
                            db_sender
                            .send(DBMessage::CreateUrlMap { url_map, resp: tx })
                            .await, "Database", "CreateUrlMap");
                        let result = rx.await.unwrap();
                        server_receiver_dropped!(resp.send(result), "ServerCreateUrlMap");
                    }
                    ReactorMessage::ServerUpdateUrlMap { key, url, resp } => {
                        let (utx, urx) = oneshot::channel();
                        didnt_receive!(
                            inner_sender
                            .clone()
                            .send(ReactorMessage::ServerGetUrlMap {
                                key,
                                resp: utx,
                            })
                            .await, "Reactor", "ServerGetUrlMap");
                        let url_map = urx.await.unwrap();
                        let mut url_map = match url_map {
                            Ok(t) => t,
                            Err(e) => {
                                server_receiver_dropped!(resp.send(Err(e)), "ServerUpdateUrlMap");
                                return;
                            }
                        };
                        url_map.url = url;
                        let (tx, rx) = oneshot::channel();
                        didnt_receive!(
                            db_sender
                            .send(DBMessage::UpdateUrlMap { url_map, resp: tx })
                            .await, "Database", "UpdateUrlMap");
                        let result = rx.await.unwrap();
                        server_receiver_dropped!(resp.send(result), "ServerUpdateUrlMap");
                    }
                    ReactorMessage::ServerDeleteUrlMap { key, resp } => {
                        let (tx, rx) = oneshot::channel();
                        didnt_receive!(
                            db_sender
                            .send(DBMessage::DeleteUrlMap { key, resp: tx })
                            .await, "Database", "UpdateUrlMap");
                        let result = rx.await.unwrap();
                        server_receiver_dropped!(resp.send(result), "ServerDeleteUrlMap");
                    }
                }
            });
        }
    }
}
