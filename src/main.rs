use anyhow::{Result, bail};

#[tokio::main]
async fn main() -> Result<()> {
    let base: &str = "http://127.0.0.1:8080";
    let user = std::env::var("TICK_USER")?;
    let pass = std::env::var("TICK_PASS")?;

    let client = reqwest::Client::builder().cookie_store(true).build()?;

    let response = client
        .post(format!("{base}/api/v2/auth/login"))
        .form(&[("username", user.as_str()), ("password", pass.as_str())])
        .send()
        .await?;

    if !response.status().is_success() {
        bail!("Login failed: {}", response.status());
    }

    let version = client
        .get(format!("{base}/api/v2/app/webapiVersion"))
        .send()
        .await?
        .text()
        .await?;

    println!("qBittorrent API: {version}");
    Ok(())
}
