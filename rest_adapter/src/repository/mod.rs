use crate::prelude_internal::*;
use crate::uri_builder::UriBuilder;
use crate::http_client::HttpClientTrait;
use crate::{Page, Content};
use crate::error::*;
use serde::export::PhantomData;
use serde::de::DeserializeOwned;

pub struct Repository<'a, T, C: HttpClientTrait + Clone> {
    uri_builder: &'a UriBuilder<C>,
    config: &'a AdapterConfiguration<C>,
    p: PhantomData<T>,
}

impl<'a, T, C: HttpClientTrait + Clone> Repository<'a, T, C> {
    pub fn new(config: &'a AdapterConfiguration<C>, uri_builder: &'a UriBuilder<C>) -> Self {
        Self { uri_builder, config, p: PhantomData }
    }

    pub fn find_all<T: DeserializeOwned>(&self, resource_type: &str) -> Result<Vec<T>, Error> {
        self.config.http_client().fetch_json(self.uri_builder.build_uri_for_resource_type(resource_type)?)
    }

    pub fn find_by_identifier<T: DeserializeOwned>(&self, resource_type: &str, identifier: ID) -> Result<T, Error> {
        self.config.http_client().fetch_json(self.uri_builder.build_uri_for_resource(resource_type, identifier)?)
    }
}

impl<'a, C: HttpClientTrait + Clone> Repository<'a, Page, C> {
    pub fn find_by_start_pid(&self, pid: ID) -> Result<Page> {
        self.config.http_client().fetch_json(self.uri_builder.build_page_uri_for_pid(pid)?)
    }
}

impl<'a, C: HttpClientTrait + Clone> Repository<'a, Content, C> {
    pub fn find_by_pid(&self, pid: ID) -> Result<Vec<Content>> {
        self.config.http_client().fetch_json(self.uri_builder.build_content_uri_for_pid(pid)?)
    }
}
