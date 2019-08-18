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

impl<C> RestAdapter<C> where C: HttpClientTrait + Clone {
    pub fn new(config: AdapterConfiguration<C>) -> Self {
        let uri_builder = UriBuilder::new(&config);

        Self { uri_builder, config }
    }
}

impl<C> AdapterTrait for RestAdapter<C> where C: HttpClientTrait + Clone {
    fn find_all<T>(&self, resource_type: &str) -> Result<Vec<T>, Error> where T: DeserializeOwned {
        self.config.http_client().fetch_json(self.uri_builder.build_uri_for_resource_type(resource_type)?)
    }

    fn find_by_identifier<T>(&self, resource_type: &str, identifier: ID) -> Result<T, Error> where T: DeserializeOwned {
        self.config.http_client().fetch_json(self.uri_builder.build_uri_for_resource(resource_type, identifier)?)
    }
}
