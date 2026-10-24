use anyhow::{Context, Result, bail};
use serde::Deserialize;
use std::os::unix::fs::PermissionsExt;

#[derive(Deserialize)]
pub struct Config {
    pub url: String,
    pub username: String,
    pub password: String,
}

fn is_private(mode: u32) -> bool {
    mode & 0o077 == 0
}

pub fn load_config() -> Result<Config> {
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

    Ok(cfg)
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
