pub mod http {
    use crate::types::Error;

    const USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

    pub async fn get(url: &str, client: Option<&reqwest::Client>) -> Result<Vec<u8>, Error> {
        log::debug!("[GET] Request on {url}");
        let client = match client {
            Some(c) => c,
            None => &reqwest::Client::builder().build()?,
        };

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("User-Agent", USER_AGENT.parse().unwrap());

        let request = client.request(reqwest::Method::GET, url).headers(headers);
        let response = request.send().await?;

        let body = response.bytes().await?.to_vec();

        Ok(body)
    }
}
