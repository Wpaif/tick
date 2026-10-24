use anyhow::{Context, Result, bail};
use serde::Deserialize;
use std::os::unix::fs::PermissionsExt;

#[derive(Deserialize)]
struct Config {
    url: String,
    username: String,
    password: String,
}

fn is_private(mode: u32) -> bool {
    mode & 0o077 == 0
}

#[tokio::main]
async fn main() -> Result<()> {
    let home = std::env::var("HOME").context("HOME is not set")?;
    let path = format!("{}/.config/tick/config.toml", home);
    let text = std::fs::read_to_string(&path).with_context(|| format!("failed to read {path}"))?;
    let mode = std::fs::metadata(&path)
        .with_context(|| format!("failet to read {path}"))?
        .permissions()
        .mode();

    if !is_private(mode) {
        bail!("{path} must not be readable by other user (run: chmod 600 {path})");
    }
    let cfg: Config = toml::from_str(&text).with_context(|| format!("failed to parse {path}"))?;

    let client = reqwest::Client::builder().cookie_store(true).build()?;

    let response = client
        .post(format!("{}/api/v2/auth/login", cfg.url))
        .form(&[("username", cfg.username), ("password", cfg.password)])
        .send()
        .await?;

    if !response.status().is_success() {
        bail!("Login failed: {}", response.status());
    }

    let version = client
        .get(format!("{}/api/v2/app/webapiVersion", cfg.url))
        .send()
        .await?
        .text()
        .await?;

    println!("qBittorrent API: {version}");
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
}
