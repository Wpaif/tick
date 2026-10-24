use anyhow::{Result, bail};
use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    url: String,
    username: String,
    password: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let text = std::fs::read_to_string("/home/wilian/.config/tick/config.toml")?;

    let cfg: Config = toml::from_str(&text)?;

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
