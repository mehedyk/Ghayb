use reqwest::{Client, Error};

pub fn setup_secure_client() -> Result<Client, Error> {
    let client = Client::builder()
        .use_rustls_tls()
        .danger_accept_invalid_certs(true)
        .build();

    client
}
