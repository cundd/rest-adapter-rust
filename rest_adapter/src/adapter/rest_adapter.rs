use crate::error::*;
use crate::{ID, AdapterConfiguration};
use crate::uri_builder::UriBuilder;
use serde::de::DeserializeOwned;
use crate::AdapterTrait;
use crate::http_client::HttpClientTrait;

pub struct RestAdapter<C: HttpClientTrait + Clone> {
    uri_builder: UriBuilder<C>,
    config: AdapterConfiguration<C>,
}

impl<C: HttpClientTrait + Clone> RestAdapter<C> {
    pub fn new(config: AdapterConfiguration<C>) -> Self {
        RestAdapter { config: config.clone(), uri_builder: UriBuilder::new(config) }
    }
}

impl<C: HttpClientTrait + Clone> AdapterTrait for RestAdapter<C> {
    fn find_all<T: DeserializeOwned>(&self, resource_type: &str) -> Result<Vec<T>, Error> {
        self.config.http_client().fetch_json(self.uri_builder.build_uri_for_resource_type(resource_type)?)
    }

    fn find_by_identifier<T: DeserializeOwned>(&self, resource_type: &str, identifier: ID) -> Result<T, Error> {
        self.config.http_client().fetch_json(self.uri_builder.build_uri_for_resource(resource_type, identifier)?)
    }
}
