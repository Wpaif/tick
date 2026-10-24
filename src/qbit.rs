use crate::config::Config;
use anyhow::{Result, bail};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Torrent {
    pub hash: String,
    pub name: String,
    pub progress: f64,
    pub dlspeed: u64,
    pub eta: i64,
    pub state: String,
}

pub async fn login(cfg: &Config) -> Result<reqwest::Client> {
    let client = reqwest::Client::builder().cookie_store(true).build()?;

    let response = client
        .post(format!("{}/api/v2/auth/login", cfg.url))
        .form(&[
            ("username", cfg.username.as_str()),
            ("password", cfg.password.as_str()),
        ])
        .send()
        .await?;

    if !response.status().is_success() {
        bail!("login failed: {}", response.status());
    }
    Ok(client)
}

#[cfg(test)]
mod tests {
    use super::*;

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
