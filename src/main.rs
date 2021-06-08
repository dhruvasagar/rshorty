use std::process;
use server::Server;
use tracing::subscriber::set_global_default;
use tracing_subscriber::FmtSubscriber;
use anyhow::{Result, Context};
use crate::db::DB;

#[macro_use]
mod macros;

mod db;
mod config;
mod models;
mod server;

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = FmtSubscriber::new();
    set_global_default(subscriber).expect("Failed to set subscriber");


    let db = DB::new().await.context("Unable to connect to database")?;
    let (db_tx, db_rx) = tokio::sync::mpsc::channel(32);

    tokio::spawn(async move {
        let mut db_manager = db::DBManager::new(db, db_rx);
        db_manager.listen().await;
    });

    tokio::spawn(async move {
        if cfg!(unix) {
            use tokio::signal::unix::*;
            let mut hup = signal(SignalKind::hangup()).unwrap();
            let mut int = signal(SignalKind::interrupt()).unwrap();
            let mut quit = signal(SignalKind::quit()).unwrap();
            let mut term = signal(SignalKind::terminate()).unwrap();

            tokio::select! {
                v = hup.recv() => v.unwrap(),
                v = int.recv() => v.unwrap(),
                v = quit.recv() => v.unwrap(),
                v = term.recv() => v.unwrap(),
            }

            println!("Goodbye!");
            process::exit(0);
        }
    });

    Server::new(db_tx).listen().await?;

    Ok(())
}
