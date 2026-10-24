use anyhow::{Context, Result, bail};
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

fn is_private(mode: u32) -> bool {
    mode & 0o077 == 0
}

#[tokio::main]
async fn main() -> Result<()> {
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
            torrent.progress,
            torrent.dlspeed,
            torrent.eta,
            torrent.state
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
}
