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
    // ---
    let args = Args::parse();
    let addr = format!("127.0.0.1:{}", args.port);
    let stream = TcpStream::connect(addr.to_socket_addrs()?.next().unwrap())
        .await
        .context("Failed to connect to fire-control server")?;

    let (reader, mut writer) = stream.into_split();
    let reader = BufReader::new(reader);
    let mut lines = reader.lines();

    let commands = [
        "10\n",    // should be scheduled
        "3\n",     // replaces previous
        "-1\n",    // cancels
        "hello\n", // invalid
        "0\n",     // invalid
        "5\n",     // should fire
    ];

    for cmd in &commands {
        print!("> {}", cmd);
        io::stdout().flush()?;
        writer.write_all(cmd.as_bytes()).await?;
        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    // Read any responses for a few seconds
    let mut response_buf = vec![];
    let timeout = tokio::time::sleep(Duration::from_secs(3));
    tokio::pin!(timeout);

    loop {
        tokio::select! {
            res = lines.next_line() => {
                if let Some(line) = res? {
                    println!("< {}", line);
                    response_buf.push(line);
                } else {
                    break;
                }
            }
            _ = &mut timeout => break,
        }
    }

    Ok(())
}
