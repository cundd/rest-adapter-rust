use super::uri_builder::UriBuilder;
use super::AdapterConfiguration;
//use super::content::ContentRepository;
//use super::page::PageRepository;
use crate::http_client::HttpClientTrait;
use crate::repository::Repository;
use crate::{Content, Page};

pub struct Factory<C: HttpClientTrait + Clone> {
    uri_builder: UriBuilder<C>,
    config: AdapterConfiguration<C>,
}

impl<C: HttpClientTrait + Clone> Factory<C> {
    pub fn new(config: &AdapterConfiguration<C>) -> Self {
        Factory {
            config: config.clone(),
            uri_builder: UriBuilder::new(config.clone()),
        }
    }

    pub fn create_repository<T>(&self) -> Repository<T, C> {
        Repository::new(&self.config, &self.uri_builder)
    }

    pub fn create_content_repository(&self) -> Repository<Content, C> {
        Repository::new(&self.config, &self.uri_builder)
    }

    pub fn create_page_repository(&self) -> Repository<Page, C> {
        Repository::new(&self.config, &self.uri_builder)
    }
}
