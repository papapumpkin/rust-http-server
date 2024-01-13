use std::io::{self};
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::mpsc;

mod cli;
mod config;
mod connection;
mod file;
mod http;
mod request;
mod response;
mod shutdown;

use config::Settings;
use connection::handle_connection;
use http::{HTTPBody, HTTPStatus};
use shutdown::ShutdownSignal;

#[tokio::main]
async fn main() -> io::Result<()> {
    // load application configuration and create Arc so it can be shared safely amongst threads
    let settings: Arc<Settings> = Arc::new(
        Settings::load()
            .await
            .expect("Failed to load configuration!"),
    );
    let address: String = format!("{}:{}", settings.hostname, settings.port);

    println!("Starting server...");
    let listener: TcpListener = TcpListener::bind(&address).await.unwrap();
    println!("Server listening on {}", address);

    // open a channel for main thread to listen for shutdown signal
    let (tx, mut rx) = mpsc::channel::<ShutdownSignal>(1);

    //Spawn a separate task to handle user shutdown logic
    let ctrl_c_tx = tx.clone();
    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to listen for ctrl_c");
        ctrl_c_tx.send(ShutdownSignal::NormalExit).await.unwrap();
    });

    // Handle SIGTERM and SIGHUP in Unix-like systems
    #[cfg(unix)]
    {
        let sigterm_tx = tx.clone();
        let sighup_tx = tx.clone();
        tokio::spawn(async move {
            use tokio::signal::unix::{signal, SignalKind};

            let mut term_signal =
                signal(SignalKind::terminate()).expect("Failed to set SIGTERM handler");
            let mut hup_signal =
                signal(SignalKind::hangup()).expect("Failed to set SIGHUP handler");

            tokio::select! {
                _ = term_signal.recv() => {
                    sigterm_tx.send(ShutdownSignal::NormalExit).await.unwrap();
                }
                _ = hup_signal.recv() => {
                    sighup_tx.send(ShutdownSignal::ReloadConfig).await.unwrap();
                    // You might handle SIGHUP differently, such as reloading config
                }
            }
        });
    }

    let mut should_exit = false;
    let mut exit_code: Option<i32> = None;

    while !should_exit {
        // simultaneously wait for connections AND check rx channel for shutdown messages
        tokio::select! {
            accept_result = listener.accept() => {
                let (socket, _) = accept_result.unwrap();
                let settings_clone = settings.clone();
                tokio::spawn(async move {
                    if let Err(e) = handle_connection(socket, settings_clone).await {
                        eprintln!("Failed to handle connection: {}", e);
                    }
                });
            }
            shutdown_signal = rx.recv() => {
                match shutdown_signal {
                    Some(ShutdownSignal::NormalExit) => {
                        println!("Shutting down normally.");
                        should_exit = true;
                    },
                    Some(ShutdownSignal::ErrorExit(code)) => {
                        eprintln!("Shutting down with error code: {}", code);
                        should_exit = true;
                        exit_code = Some(code);
                    },
                    Some(ShutdownSignal::ReloadConfig) => {
                        println!("Reloading configuration.");
                        break;
                    },
                    None => should_exit = true, // Channel closed
                }
            }
        }
    }
    if let Some(code) = exit_code {
        std::process::exit(code);
    }

    Ok(())
}
