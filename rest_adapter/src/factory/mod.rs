use super::uri_builder::UriBuilder;
use super::AdapterConfiguration;
use crate::http_client::HttpClientTrait;
use crate::repository::Repository;
use crate::{Content, Page};
use serde::de::DeserializeOwned;

/// Factory for repositories
///
/// ```
/// use rest_adapter::*;
///
/// {
///     let config = AdapterConfiguration::from_url("http://base.url.tld/rest/").unwrap();
///     let content_repository = Factory::new(config).create_content_repository();
/// }
/// {
///     let config = AdapterConfiguration::from_url("http://base.url.tld/rest/").unwrap();
///     let page_repository = Factory::new(config).create_page_repository();
/// }
/// ```
pub struct Factory<C: HttpClientTrait + Clone> {
    uri_builder: UriBuilder,
    config: AdapterConfiguration<C>,
}

impl<C: HttpClientTrait + Clone> Factory<C> {
    pub fn new(config: AdapterConfiguration<C>) -> Self {
        let uri_builder = UriBuilder::new(&config);

        Factory { config, uri_builder }
    }

    pub fn create_repository<T: DeserializeOwned>(&self) -> Repository<T, C> {
        Repository::new(&self.config, &self.uri_builder)
    }

    pub fn create_content_repository(&self) -> Repository<Content, C> {
        Repository::new(&self.config, &self.uri_builder)
    }

    pub fn create_page_repository(&self) -> Repository<Page, C> {
        Repository::new(&self.config, &self.uri_builder)
    }
}
