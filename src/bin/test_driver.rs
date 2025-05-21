use anyhow::{Context, Result};
use clap::Parser;
use std::io::{self, Write};
use std::net::ToSocketAddrs;
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;

#[derive(Parser, Debug)]
#[command(name = "test-driver")]
#[command(about = "Send commands to fire-control TCP server")]
struct Args {
    /// Port of the fire-control server (default: 8124)
    #[arg(short, long, default_value_t = 8124)]
    port: u16,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let addr = format!("127.0.0.1:{}", args.port);
    let stream = TcpStream::connect(addr.to_socket_addrs()?.next().unwrap())
        .await
        .context("Failed to connect to fire-control server")?;

    let (reader, mut writer) = stream.into_split();

    // Send test commands
    let commands = ["10\n", "3\n", "-1\n", "hello\n", "0\n", "5\n"];

    for cmd in &commands {
        print!("> {}", cmd);
        io::stdout().flush()?;
        writer.write_all(cmd.as_bytes()).await?;
        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    // ✅ Close the writer to signal end-of-input
    writer.shutdown().await?;
    println!("✅ All commands sent. Waiting for response...");

    // TODO(johnb): Replace this test driver with an integration test in tests/tcp_integration.rs (firing once, no sleeps, use next_line)

    let mut reader = BufReader::new(reader);
    let mut buffer = String::new();

    while reader.read_line(&mut buffer).await? != 0 {
        print!("< {}", buffer);
        buffer.clear();
    }

    tokio::time::sleep(Duration::from_millis(200)).await;
    Ok(())
}
