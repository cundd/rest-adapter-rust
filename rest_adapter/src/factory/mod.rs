use super::uri_builder::UriBuilder;
use super::AdapterConfiguration;
use super::content::ContentRepository;
use super::page::PageRepository;

pub struct Factory {
    uri_builder: UriBuilder,
    #[allow(unused)]
    config: AdapterConfiguration,
}

impl Factory {
    pub fn new(config: &AdapterConfiguration) -> Self {
        Factory { config: config.clone(), uri_builder: UriBuilder::new(config.clone()) }
    }

    pub fn create_content_repository(&self) -> ContentRepository {
        ContentRepository::new(&self.uri_builder)
    }

    pub fn create_page_repository(&self) -> PageRepository {
        PageRepository::new(&self.uri_builder)
    }
}
