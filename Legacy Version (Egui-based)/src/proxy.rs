use reqwest::{Client, Proxy};
use std::error::Error;

pub fn setup_proxy() -> Result<Client, Box<dyn Error>> {
    let proxy = Proxy::all("http://127.0.0.1:8080")?;  // Use your proxy address here

    let client = Client::builder()
        .proxy(proxy)
        .danger_accept_invalid_certs(true)
        .use_rustls_tls()
        .build()?;

    Ok(client)
}
