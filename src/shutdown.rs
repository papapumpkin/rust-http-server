use tokio::sync::mpsc;

pub enum ShutdownSignal {
    NormalExit,
    ErrorExit(i32),
    ReloadConfig,
}

pub async fn handle_shutdown_signals(tx: mpsc::Sender<ShutdownSignal>) {
    // CTRL-C handler
    let ctrl_c_tx = tx.clone();
    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to listen for ctrl_c");
        ctrl_c_tx
            .send(ShutdownSignal::NormalExit)
            .await
            .expect("Failed to send shutdown signal");
    });

    // SIGTERM and SIGHUP handlers for Unix systems
    #[cfg(unix)]
    setup_unix_signal_handlers(tx).await;
}

#[cfg(unix)]
async fn setup_unix_signal_handlers(tx: mpsc::Sender<ShutdownSignal>) {
    use tokio::signal::unix::{signal, SignalKind};

    let sigterm_tx = tx.clone();
    let sighup_tx = tx.clone();
    tokio::spawn(async move {
        let mut term_signal =
            signal(SignalKind::terminate()).expect("Failed to set SIGTERM handler");
        let mut hup_signal = signal(SignalKind::hangup()).expect("Failed to set SIGHUP handler");

        tokio::select! {
            _ = term_signal.recv() => {
                sigterm_tx.send(ShutdownSignal::NormalExit).await.expect("Failed to send SIGTERM signal");
            }
            _ = hup_signal.recv() => {
                sighup_tx.send(ShutdownSignal::ReloadConfig).await.expect("Failed to send SIGHUP signal");
            }
        }
    });
}
