use crate::error::*;
use crate::{ID, AdapterConfiguration};
use crate::uri_builder::UriBuilder;
use serde::de::DeserializeOwned;
use crate::AdapterTrait;
use crate::http_client::HttpClientTrait;

/// General adapter to perform REST requests
///
/// ```
/// use rest_adapter::*;
/// use serde::de::DeserializeOwned;
///
/// fn load_records<T: DeserializeOwned>() -> Result<Vec<T>>{
///     let config = AdapterConfiguration::from_url("http://base.url.tld/rest/")?;
///     let rd = RestAdapter::new(config);
///
///     rd.find_all::<T>("Vendor-RealEstate-Person")
/// }
/// ```
pub struct RestAdapter<C: HttpClientTrait + Clone> {
    uri_builder: UriBuilder,
    config: AdapterConfiguration<C>,
}

impl<C: HttpClientTrait + Clone> RestAdapter<C> {
    pub fn new(config: AdapterConfiguration<C>) -> Self {
        let uri_builder = UriBuilder::new(&config);

        Self { uri_builder, config }
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
