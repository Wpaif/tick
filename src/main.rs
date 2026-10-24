mod config;
mod format;
mod qbit;
use anyhow::Result;
use clap::{Parser, Subcommand};
use config::load_config;
use format::{format_eta, format_progress, format_speed, format_state};
use qbit::{Torrent, login};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// List all torrents
    List,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::List => list().await,
    }
}

async fn list() -> Result<()> {
    let cfg = load_config()?;
    let client = login(&cfg).await?;

    let body = client
        .get(format!("{}/api/v2/torrents/info", cfg.url))
        .send()
        .await?
        .text()
        .await?;

    let torrents: Vec<Torrent> = serde_json::from_str(&body)?;

    println!("{} torrents", torrents.len());
    for torrent in &torrents {
        println!(
            "{}\n  hash: {}\n  progress: {}\n  dlspeed: {}\n  eta: {}\n  state: {}\n",
            torrent.name,
            torrent.hash,
            format_progress(torrent.progress),
            format_speed(torrent.dlspeed),
            format_eta(torrent.eta),
            format_state(&torrent.state)
        );
    }
    Ok(())
}
