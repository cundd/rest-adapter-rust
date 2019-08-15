use super::error::*;
use url::Url;
use reqwest::Response;
use serde::de::DeserializeOwned;

pub fn fetch<T: DeserializeOwned>(url: Url) -> Result<T> {
    let mut response = reqwest::Client::new()
        .get(url.clone())
        .send()?;

    validate_status_code(&url, &mut response)?;

    match response.json() {
        Ok(contents) => Ok(contents),
        Err(e) => Err(Error::data_error_with_url(e, url.into_string()))
    }
}

fn validate_status_code(url: &Url, response: &mut Response) -> Result<()> {
    let status_code = response.status().as_u16();

    if status_code < 400 {
        Ok(())
    } else if status_code < 500 {
        match response.text() {
            Ok(body) => Err(Error::bad_request_error(format!("Invalid request to {}: {}", url, body))),
            Err(_) => Err(Error::bad_request_error(format!("Invalid request to {}", url)))
        }
    } else {
        Err(Error::server_error(format!("Server error for request to {}", url)))
    }
}
