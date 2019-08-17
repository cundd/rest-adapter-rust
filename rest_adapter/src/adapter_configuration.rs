use url::Url;
use crate::error::Result;
use crate::http_client::{HttpClientTrait, default_client, DefaultClient};

#[derive(Clone)]
pub struct AdapterConfiguration<C: HttpClientTrait + Clone> {
    base_uri: Url,
    http_client: Box<C>,
}

impl<C: HttpClientTrait + Clone> AdapterConfiguration<C> {
    pub fn from_url_and_client<S: Into<String>>(base_uri: S, http_client: C) -> Result<Self> {
        let base_uri_prepared = base_uri.into().trim().trim_end_matches('/').to_string() + "/";

        Ok(AdapterConfiguration {
            base_uri: Url::parse(&base_uri_prepared)?,
            http_client: Box::new(http_client),
        })
    }

    pub fn base_uri(&self) -> &Url {
        &self.base_uri
    }
    pub fn http_client(&self) -> &C {
        &self.http_client
    }
}

impl AdapterConfiguration<DefaultClient> {
    pub fn from_url<S: Into<String>>(base_uri: S) -> Result<Self> {
        Self::from_url_and_client(base_uri, default_client())
    }
}
