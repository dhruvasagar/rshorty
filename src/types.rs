// use crate::{db::DBMessage, reactor::ReactorMessage, server::ServerMessage};

pub type OneShotMessageResponse<T> = tokio::sync::oneshot::Sender<T>;

// pub type DBSender = tokio::sync::mpsc::Sender<DBMessage>;
// pub type ReactorSender = tokio::sync::mpsc::Sender<ReactorMessage>;
// pub type ReactorReceiver = tokio::sync::mpsc::Receiver<ReactorMessage>;
// pub type ServerReceiver = tokio::sync::mpsc::Receiver<ServerMessage>;
