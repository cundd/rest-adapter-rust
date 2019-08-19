use crate::error::*;
use url::Url;

/// Definition of a HTTP client
pub trait HttpClientTrait {
    /// Return a new client instance
    fn new() -> Self;

    /// Fetch the HTTP response body from the webservice
    fn fetch(&self, url: &Url) -> Result<String>;
}
