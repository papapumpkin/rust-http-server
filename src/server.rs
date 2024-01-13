use crate::{config::Settings, connection::handle_connection, shutdown::ShutdownSignal};
use std::io::{self};
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;

pub struct Server {
    settings: Arc<Settings>,
    listener: TcpListener,
    rx: mpsc::Receiver<ShutdownSignal>,
}

impl Server {
    pub async fn new(
        settings: Arc<Settings>,
        rx: mpsc::Receiver<ShutdownSignal>,
    ) -> io::Result<Self> {
        let address = format!("{}:{}", settings.hostname, settings.port);
        let listener = TcpListener::bind(&address).await?;
        println!("Server listening on {}", address);

        Ok(Server {
            settings,
            listener,
            rx,
        })
    }

    pub async fn run(&mut self) -> Option<i32> {
        let exit_code: Option<i32> = None;

        loop {
            tokio::select! {
                accept_result = self.listener.accept() => {
                    if let Ok((socket, _)) = accept_result {
                        self.handle_incoming_connection(socket).await;
                    }
                }
                shutdown_signal = self.rx.recv() => {
                    if self.process_shutdown_signal(shutdown_signal).await {
                        break;
                    }
                }
            }
        }

        exit_code
    }

    async fn handle_incoming_connection(&self, socket: TcpStream) {
        let settings_clone = self.settings.clone();
        tokio::spawn(async move {
            if let Err(e) = handle_connection(socket, settings_clone).await {
                eprintln!("Failed to handle connection: {}", e);
            }
        });
    }

    async fn process_shutdown_signal(&mut self, shutdown_signal: Option<ShutdownSignal>) -> bool {
        match shutdown_signal {
            Some(ShutdownSignal::NormalExit) => {
                println!("Shutting down normally.");
                true
            }
            Some(ShutdownSignal::ErrorExit(code)) => {
                eprintln!("Shutting down with error code: {}", code);
                true
            }
            Some(ShutdownSignal::ReloadConfig) => {
                println!("Reloading configuration.");
                false // Indicates not to exit
            }
            None => true, // Channel closed
        }
    }
}
