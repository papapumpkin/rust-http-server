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
                let new_settings: Arc<Settings> = Arc::new(
                    Settings::load()
                        .await
                        .expect("Failed to reload settings!"),
                );
                self.reload_server(new_settings).await;
                false // Indicates not to exit
            }
            None => true, // Channel closed
        }
    }

    pub async fn reload_server(&mut self, new_settings: Arc<Settings>) {
        println!("Server reload triggered!");
        self.settings = new_settings;
        let address = format!("{}:{}", self.settings.hostname, self.settings.port);
        self.listener = TcpListener::bind(&address).await.unwrap();
        println!("Server reinitialized successfully, now listening on {}", address);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_server_new() {
        let hostname: String = "127.0.0.1".to_string();
        let port: String = "0".to_string();
        let settings = Arc::new(Settings {
            hostname: hostname,
            port: port,
            buffer_size: 1024,
        });
        let (_tx, rx) = mpsc::channel(1); // Create a mock channel

        let server = Server::new(settings, rx).await;
        assert!(server.is_ok());
    }

    #[tokio::test]
    async fn test_process_shutdown_signal_normal() {
        let hostname: String = "127.0.0.1".to_string();
        let port: String = "0".to_string();
        let settings = Arc::new(Settings {
            hostname: hostname,
            port: port,
            buffer_size: 1024,
        });
        let (tx, rx) = mpsc::channel(1);
        let mut server = Server::new(settings, rx).await.unwrap();

        tx.send(ShutdownSignal::NormalExit).await.unwrap();
        assert!(
            server
                .process_shutdown_signal(Some(ShutdownSignal::NormalExit))
                .await
        );
    }

    #[tokio::test]
    async fn test_process_shutdown_signal_error() {
        let hostname: String = "127.0.0.1".to_string();
        let port: String = "0".to_string();
        let settings = Arc::new(Settings {
            hostname: hostname,
            port: port,
            buffer_size: 1024,
        });
        let (tx, rx) = mpsc::channel(1);
        let mut server = Server::new(settings, rx).await.unwrap();

        tx.send(ShutdownSignal::ErrorExit(1)).await.unwrap();
        assert!(
            server
                .process_shutdown_signal(Some(ShutdownSignal::ErrorExit(1)))
                .await
        );
    }

    #[tokio::test]
    async fn test_process_shutdown_signal_reload() {
        let initial_settings = Arc::new(Settings {
            hostname: "127.0.0.1".to_string(),
            port: "0".to_string(),
            buffer_size: 1024,
        });
        let (tx, rx) = mpsc::channel(1);
        let mut server = Server::new(initial_settings.clone(), rx).await.unwrap();
    
        // Send the reload signal
        tx.send(ShutdownSignal::ReloadConfig).await.unwrap();
    
        // Create new settings to simulate a reload
        let reloaded_settings = Arc::new(Settings {
            hostname: "127.0.0.1".to_string(),
            port: "1234".to_string(), // Change some settings to test reload
            buffer_size: 2048,
        });
    
        // Call the method to reload settings
        server.reload_server(reloaded_settings.clone()).await;
    
        // Assert that settings were reloaded
        assert_eq!(server.settings.port, reloaded_settings.port);
        assert_eq!(server.settings.buffer_size, reloaded_settings.buffer_size);
    
    }
}
