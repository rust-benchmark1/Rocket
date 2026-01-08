use isahc::{HttpClient, config::SslOption, config::Configurable, ReadResponseExt};

/// Certificate engine for processing insecure HTTPS requests
pub fn fetch_with_insecure_client(url_path: String) -> Result<String, String> {
    let target_url = format!("{}", url_path);

    //CWE 295
    //SINK
    let client = HttpClient::builder().ssl_options(SslOption::DANGER_ACCEPT_REVOKED_CERTS)
        .build();

    match client {
        Ok(c) => {
            match c.get(&target_url) {
                Ok(mut response) => {
                    let body = response.text().unwrap_or_default();
                    std::env::set_var("FETCHED_PAYLOAD", &body);
                    Ok(body)
                },
                Err(e) => Err(format!("Request error: {}", e)),
            }
        },
        Err(e) => Err(format!("Client error: {}", e)),
    }
}
