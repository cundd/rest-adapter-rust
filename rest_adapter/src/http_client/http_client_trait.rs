use crate::error::*;
use url::Url;
use serde::de::DeserializeOwned;

pub trait HttpClientTrait {
    fn new() -> Self;

    fn fetch_json<T: DeserializeOwned>(&self, url: Url) -> Result<T> {
        match serde_json::from_str(&self.fetch(&url)?) {
            Ok(contents) => Ok(contents),
            Err(e) => Err(Error::data_error_with_url(&e, url.into_string()))
        }
    }

    fn fetch(&self, url: &Url) -> Result<String>;
}
