use reqwest::Client;
use std::error::Error;

pub async fn check_url(url: &str) -> Result<String, Box<dyn Error>> {
    let client = Client::builder()
        .use_rustls_tls()
        .danger_accept_invalid_certs(true)
        .build()?;

    let response = client.get(format!("https://{}", url))
        .send()
        .await?;

    if response.status().is_success() {
        Ok("Successfully reached the domain".to_string())
    } else {
        Ok(format!(
            "Failed to reach the domain. Status: {}",
            response.status()
        ))
    }
}
