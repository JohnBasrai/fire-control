use std::net::SocketAddr;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tracing::{error, info, warn};

use crate::command::FireCommand;
use crate::controller::FiringController;

pub async fn start_tcp_server(port: u16, controller: FiringController) -> anyhow::Result<()> {
    // ---
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = TcpListener::bind(addr).await?;
    info!("🚪 Listening on {}", addr);

    loop {
        match listener.accept().await {
            Ok((stream, peer)) => {
                let controller = controller.clone();
                info!(%peer, "🔌 Accepted connection");
                tokio::spawn(async move {
                    if let Err(e) = handle_client(stream, controller).await {
                        warn!(%peer, "❌ Client error: {:?}", e);
                    }
                    info!(%peer, "🔌 Connection closed");
                });
            }
            Err(e) => {
                error!("💥 Failed to accept connection: {:?}", e);
            }
        }
    }
}

async fn handle_client(stream: TcpStream, controller: FiringController) -> anyhow::Result<()> {
    // ---
    let peer = stream.peer_addr()?;
    let reader = BufReader::new(stream);
    let mut lines = reader.lines();

    while let Some(line) = lines.next_line().await? {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        tracing::debug!(%trimmed, "Received raw line"); // 👈 ADD THIS

        match trimmed.parse::<FireCommand>() {
            Ok(cmd) => {
                info!(%peer, ?cmd, "➡️ Received command");
                controller.handle_command(cmd).await;
            }
            Err(e) => {
                warn!(%peer, input = ?trimmed, "⚠️ Invalid command: {:?}", e);
            }
        }
    }

    Ok(())
}
