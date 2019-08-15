use crate::error::*;
use crate::{ID, AdapterConfiguration};
use crate::uri_builder::UriBuilder;
use crate::request_helper::fetch;
use serde::de::DeserializeOwned;
//use crate::::AdapterTrait;
use super::adapter_trait::AdapterTrait;

pub struct RestAdapter {
    uri_builder: UriBuilder,
}

impl RestAdapter {
    pub fn new(config: AdapterConfiguration) -> Self {
        RestAdapter { uri_builder: UriBuilder::new(config) }
    }
}

impl AdapterTrait for RestAdapter {
    fn find_all<T: DeserializeOwned>(&self, resource_type: &str) -> Result<Vec<T>, Error> {
        fetch(self.uri_builder.build_uri_for_resource_type(resource_type)?)
    }

    fn find_by_identifier<T: DeserializeOwned>(&self, resource_type: &str, identifier: ID) -> Result<T, Error> {
        fetch(self.uri_builder.build_uri_for_resource(resource_type, identifier)?)
    }
}
