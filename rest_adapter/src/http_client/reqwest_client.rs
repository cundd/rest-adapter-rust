use url::Url;
use reqwest::Response;
use log::debug;
use crate::error::*;
use crate::http_client::http_client_trait::HttpClientTrait;

#[derive(Clone)]
pub struct ReqwestClient {}

impl HttpClientTrait for ReqwestClient {
    fn new() -> Self {
        ReqwestClient {}
    }

    fn fetch(&self, url: &Url) -> Result<String> {
        debug!("Make request for '{}'", url);
        let mut response = reqwest::Client::new()
            .get(url.clone())
            .send()?;

        validate_status_code(&url, &mut response)?;

        Ok(response.text()?)
    }
}

fn validate_status_code(url: &Url, response: &mut Response) -> Result<()> {
    let status_code = response.status().as_u16();

    if status_code < 400 {
        Ok(())
    } else if status_code == 404 {
        match response.text() {
            Ok(body) => Err(Error::not_found_error(format!("Resource at {} not found: {}", url, body))),
            Err(_) => Err(Error::not_found_error(format!("Resource at {} not found", url)))
        }
    } else if status_code < 500 {
        match response.text() {
            Ok(body) => Err(Error::bad_request_error(format!("Invalid request to {}: {}", url, body))),
            Err(_) => Err(Error::bad_request_error(format!("Invalid request to {}", url)))
        }
    } else {
        Err(Error::server_error(format!("Server error for request to {}", url)))
    }
}
