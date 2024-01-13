use std::io::{self};
use std::sync::Arc;
use tokio::sync::mpsc;

mod cli;
mod config;
mod connection;
mod file;
mod http;
mod request;
mod response;
mod server;
mod shutdown;

use config::Settings;
use http::{HTTPBody, HTTPStatus};
use server::Server;
use shutdown::ShutdownSignal;

#[tokio::main]
async fn main() -> io::Result<()> {
    // load application configuration and create Arc so it can be shared safely amongst threads
    let settings: Arc<Settings> = Arc::new(
        Settings::load()
            .await
            .expect("Failed to load configuration!"),
    );

    // open a channel for main thread to listen for shutdown signal
    let (tx, rx) = mpsc::channel::<ShutdownSignal>(1);
    shutdown::handle_shutdown_signals(tx).await;

    let mut server = Server::new(settings, rx).await?;
    // Run the server and handle its exit
    if let Some(exit_code) = server.run().await {
        std::process::exit(exit_code);
    }

    Ok(())
}
