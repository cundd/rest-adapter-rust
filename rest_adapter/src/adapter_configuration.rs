use url::Url;
use crate::error::Result;

#[derive(Clone)]
pub struct AdapterConfiguration {
    base_uri: Url
}

impl AdapterConfiguration {
    pub fn from_url<S: Into<String>>(base_uri: S) -> Result<Self> {
        let base_uri_prepared = base_uri.into().trim().trim_end_matches('/').to_string() + "/";

        Ok(AdapterConfiguration { base_uri: Url::parse(&base_uri_prepared)? })
    }

    pub fn base_uri(&self) -> &Url {
        &self.base_uri
    }
}
