use url::Url;
use crate::error::Result;
use crate::http_client::{HttpClientTrait, default_client, DefaultClient};

/// Container for the API configuration
///
/// The configuration contains the [`base_uri`] and the concrete [`http_client`] to use
#[derive(Clone)]
pub struct AdapterConfiguration<C: HttpClientTrait + Clone> {
    base_uri: Url,
    http_client: C,
}

impl<C: HttpClientTrait + Clone> AdapterConfiguration<C> {
    /// Return a new configuration instance with [`base_uri`] and the concrete [`http_client`]
    pub fn from_url_and_client<S: Into<String>>(base_uri: S, http_client: C) -> Result<Self> {
        let base_uri_prepared = base_uri.into().trim().trim_end_matches('/').to_string() + "/";

        Ok(AdapterConfiguration {
            base_uri: Url::parse(&base_uri_prepared)?,
            http_client,
        })
    }

    /// Return the base URI for all REST requests
    ///
    /// ```
    /// use rest_adapter::AdapterConfiguration;
    ///
    /// let config = AdapterConfiguration::from_url("http://base.url.tld/rest/");
    ///
    /// assert!(config.is_ok());
    /// assert_eq!("http://base.url.tld/rest/", config.unwrap().base_uri().as_str());
    /// ```
    pub fn base_uri(&self) -> &Url {
        &self.base_uri
    }

    /// Return the concrete HTTP client implementation to make requests
    ///
    /// The client implements [`HttpClientTrait`]
    pub fn http_client(&self) -> &C {
        &self.http_client
    }
}

/// Configuration implementation for the default HTTP client
impl AdapterConfiguration<DefaultClient> {
    /// Return a new configuration instance with the given [`base_uri`] and the default HTTP client
    pub fn from_url<S: Into<String>>(base_uri: S) -> Result<Self> {
        Self::from_url_and_client(base_uri, default_client())
    }
}
