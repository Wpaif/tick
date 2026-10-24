use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand};
use serde::Deserialize;
use std::os::unix::fs::PermissionsExt;

#[derive(Deserialize)]
struct Config {
    url: String,
    username: String,
    password: String,
}

#[derive(Deserialize)]
struct Torrent {
    hash: String,
    name: String,
    progress: f64,
    dlspeed: u64,
    eta: i64,
    state: String,
}

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

fn is_private(mode: u32) -> bool {
    mode & 0o077 == 0
}

fn format_progress(progress: f64) -> String {
    format!("{:.0}%", progress * 100.0)
}

fn format_speed(speed: u64) -> String {
    if speed < 1024 {
        return format!("{} B/s", speed);
    }
    if speed < 1024 * 1024 {
        return format!("{:.1} KiB/s", speed as f64 / 1024.0);
    }
    format!("{:.1} MiB/s", speed as f64 / (1024.0 * 1024.0))
}

fn format_eta(eta: i64) -> String {
    if eta >= 8640000 {
        return "∞".to_string();
    }
    if eta < 60 {
        return format!("{}s", eta);
    }
    if eta < 3600 {
        return format!("{}m", eta / 60);
    }
    if eta < 3600 * 24 {
        return format!("{}h", eta / 3600);
    }
    format!("{}d", eta / (3600 * 24))
}

fn format_state(state: &str) -> &str {
    match state {
        "downloading" | "forcedDL" => "Downloading",
        "stoppedUP" | "stoppedDL" => "Stopped",
        "uploading" | "forcedUP" => "Seeding",
        "stalledDL" | "stalledUP" => "Stalled",
        "queuedDL" | "queuedUP" => "Queued",
        "checkingDL" | "checkingUP" | "checkingResumeData" => "Checking",
        "metaDL" => "Fetching metadata",
        "missingFiles" => "Missing files",
        "error" => "Error",
        _ => state,
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::List => list().await,
    }
}

async fn list() -> Result<()> {
    let home = std::env::var("HOME").context("HOME is not set")?;
    let path = format!("{}/.config/tick/config.toml", home);
    let mode = std::fs::metadata(&path)
        .with_context(|| format!("failed to read {path}"))?
        .permissions()
        .mode();

    if !is_private(mode) {
        bail!("{path} must not be readable by other users (run: chmod 600 {path})");
    }
    let text = std::fs::read_to_string(&path).with_context(|| format!("failed to read {path}"))?;

    let cfg: Config = toml::from_str(&text).with_context(|| format!("failed to parse {path}"))?;

    let client = reqwest::Client::builder().cookie_store(true).build()?;

    let response = client
        .post(format!("{}/api/v2/auth/login", cfg.url))
        .form(&[("username", cfg.username), ("password", cfg.password)])
        .send()
        .await?;

    if !response.status().is_success() {
        bail!("login failed: {}", response.status());
    }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_mode_600() {
        assert!(is_private(0o600));
    }

    #[test]
    fn rejects_mode_644() {
        assert!(!is_private(0o644));
    }

    #[test]
    fn parses_torrent_list() {
        let json = r#"[
            {
                "hash": "asdfg",
                "name": "Arquivo 1",
                "progress": 1,
                "dlspeed": 0,
                "eta": 8640000,
                "state": "stoppedUP",
                "size": 123
            }
        ]"#;

        let torrents: Vec<Torrent> = serde_json::from_str(json).unwrap();

        assert_eq!(torrents.len(), 1);
        assert_eq!(torrents[0].name, "Arquivo 1");
        assert_eq!(torrents[0].progress, 1.0);
        assert_eq!(torrents[0].state, "stoppedUP");
    }

    #[test]
    fn formats_progress_as_percent() {
        assert_eq!(format_progress(0.42), "42%");
    }

    #[test]
    fn rounds_progress_to_whole_percent() {
        assert_eq!(format_progress(0.29), "29%");
    }

    #[test]
    fn formats_speed_below_1024_as_bytes() {
        assert_eq!(format_speed(500), "500 B/s");
    }

    #[test]
    fn formats_speed_in_kib() {
        assert_eq!(format_speed(2048), "2.0 KiB/s");
    }

    #[test]
    fn formats_speed_in_mib() {
        assert_eq!(format_speed(1048576), "1.0 MiB/s");
    }

    #[test]
    fn formats_unknown_eta_as_infinity() {
        assert_eq!(format_eta(8640000), "∞");
    }

    #[test]
    fn formats_eta_as_seconds() {
        assert_eq!(format_eta(45), "45s");
    }

    #[test]
    fn formats_eta_as_minutes() {
        assert_eq!(format_eta(360), "6m");
    }

    #[test]
    fn formats_eta_as_hours() {
        assert_eq!(format_eta(7200), "2h");
    }

    #[test]
    fn formats_eta_as_days() {
        assert_eq!(format_eta(86400), "1d");
    }

    #[test]
    fn formats_downloading_states() {
        assert_eq!(format_state("downloading"), "Downloading");
        assert_eq!(format_state("forcedDL"), "Downloading");
    }

    #[test]
    fn formats_stopped_states() {
        assert_eq!(format_state("stoppedUP"), "Stopped");
        assert_eq!(format_state("stoppedDL"), "Stopped");
    }

    #[test]
    fn formats_seeding_states() {
        assert_eq!(format_state("uploading"), "Seeding");
        assert_eq!(format_state("forcedUP"), "Seeding");
    }

    #[test]
    fn formats_stalled_states() {
        assert_eq!(format_state("stalledDL"), "Stalled");
        assert_eq!(format_state("stalledUP"), "Stalled");
    }

    #[test]
    fn formats_queued_states() {
        assert_eq!(format_state("queuedDL"), "Queued");
        assert_eq!(format_state("queuedUP"), "Queued");
    }

    #[test]
    fn formats_checking_states() {
        assert_eq!(format_state("checkingDL"), "Checking");
        assert_eq!(format_state("checkingUP"), "Checking");
        assert_eq!(format_state("checkingResumeData"), "Checking");
    }

    #[test]
    fn formats_fetching_metadata_states() {
        assert_eq!(format_state("metaDL"), "Fetching metadata");
    }

    #[test]
    fn formats_missing_files_states() {
        assert_eq!(format_state("missingFiles"), "Missing files");
    }

    #[test]
    fn formats_error_states() {
        assert_eq!(format_state("error"), "Error");
    }
}
