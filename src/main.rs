use server::Server;
use reactor::Reactor;
use std::sync::Arc;

#[macro_use]
pub mod macros;

mod db;
mod models;
mod types;
mod server;
mod reactor;

#[tokio::main]
async fn main() {
    let pool = match db::connect(&std::env::var("DATABASE_URL").unwrap()).await {
        Ok(p) => p,
        Err(_e) => {
            panic!("Database connection failed. Check if your connection URL is correct and your DB is reachable.")
        }
    };

    let (db_tx, db_rx) = tokio::sync::mpsc::channel(32);
    let (sv_tx, sv_rx) = tokio::sync::mpsc::channel(32);

    tokio::spawn(async {
        let mut db_manager = db::DBManager::new(pool, db_rx);
        db_manager.listen().await;
    });

    tokio::spawn(async move {
        let server = Server::new(sv_tx);
        let port = (std::env::var("PORT").unwrap_or("3000".to_string())).parse::<i64>().expect("PORT is not a number?");
        let host = std::env::var("HOST").unwrap_or("127.0.0.1".to_string());
        server.listen(host, port).await;
    });

    // tokio::spawn(async move {
    //     if cfg!(unix) {
    //         use tokio::signal::unix::*;
    //         let mut hup = signal(SignalKind::hangup()).unwrap();
    //         let mut int = signal(SignalKind::interrupt()).unwrap();
    //         let mut quit = signal(SignalKind::quit()).unwrap();
    //         let mut term = signal(SignalKind::terminate()).unwrap();

    //         tokio::select! {
    //             v = hup.recv() => v.unwrap(),
    //             v = int.recv() => v.unwrap(),
    //             v = quit.recv() => v.unwrap(),
    //             v = term.recv() => v.unwrap(),
    //         }

    //         println!("Goodbye!");
    //     }
    // });

    let _ = tokio::spawn(async {
        let server_receiver = Arc::new(tokio::sync::Mutex::new(sv_rx));
        let (tx, rx) = tokio::sync::mpsc::channel(128);
        let mut reactor = Reactor {
            db_sender: db_tx,
            server_receiver,
            inner_sender: tx,
        };
        reactor.listen(rx).await;
    })
    .await;
}
