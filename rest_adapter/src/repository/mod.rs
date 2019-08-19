use crate::prelude_internal::*;
use crate::uri_builder::UriBuilder;
use crate::http_client::HttpClientTrait;
use crate::{Page, Content};
use crate::error::*;
use crate::Url;
use serde::export::PhantomData;
use serde::de::DeserializeOwned;

/// Repository to fetch information from the webservice
pub struct Repository<'a, T, C>
    where T: DeserializeOwned,
          C: HttpClientTrait + Clone {
    uri_builder: &'a UriBuilder,
    config: &'a AdapterConfiguration<C>,
    target: PhantomData<T>,
}

impl<'a, T, C> Repository<'a, T, C>
    where T: DeserializeOwned,
          C: HttpClientTrait + Clone {
    pub fn new(config: &'a AdapterConfiguration<C>, uri_builder: &'a UriBuilder) -> Self {
        Self { uri_builder, config, target: PhantomData }
    }

    //noinspection DuplicatedCode
    fn fetch_and_convert<R: DeserializeOwned>(&self, url: Url) -> Result<R> {
        match serde_json::from_str(&self.config.http_client().fetch(&url)?) {
            Ok(objects) => Ok(objects),
            Err(e) => Err(Error::data_error_with_url(&e, url.into_string()))
        }
    }
}

pub trait RepositoryTrait<T>
    where T: DeserializeOwned {
    fn find_all(&self, resource_type: &str) -> Result<Vec<T>>;

    fn find_by_identifier(&self, resource_type: &str, identifier: ID) -> Result<T>;
}

impl<'a, T, C> RepositoryTrait<T> for Repository<'a, T, C>
    where T: DeserializeOwned,
          C: HttpClientTrait + Clone {
    fn find_all(&self, resource_type: &str) -> Result<Vec<T>> {
        self.fetch_and_convert(self.uri_builder.build_uri_for_resource_type(resource_type)?)
    }

    fn find_by_identifier(&self, resource_type: &str, identifier: i32) -> Result<T> {
        self.fetch_and_convert(self.uri_builder.build_uri_for_resource(resource_type, identifier)?)
    }
}

/// Special Repository implementation to fetch [`Page`s]
impl<'a, C> Repository<'a, Page, C> where C: HttpClientTrait + Clone {
    pub fn find_by_start_pid(&self, pid: ID) -> Result<Page> {
        self.fetch_and_convert(self.uri_builder.build_page_uri_for_pid(pid)?)
    }
}

/// Special Repository implementation to fetch [`Content` elements]
impl<'a, C> Repository<'a, Content, C> where C: HttpClientTrait + Clone {
    pub fn find_by_pid(&self, pid: ID) -> Result<Vec<Content>> {
        self.fetch_and_convert(self.uri_builder.build_content_uri_for_pid(pid)?)
    }
}
