use crate::Url;
use crate::prelude_internal::*;
use crate::http_client::HttpClientTrait;

pub struct UriBuilder {
    base_uri: Url,
}

impl UriBuilder {
    pub fn new<C: HttpClientTrait + Clone>(config: &AdapterConfiguration<C>) -> Self {
        Self { base_uri: config.base_uri().clone() }
    }

    pub fn build_uri(&self, /*config: &AdapterConfiguration<C>,*/ path: &str) -> Result<Url> {
        Ok(self.base_uri.join(&format!("{}/", path))?)
    }

    pub fn build_content_uri_for_pid(&self, /*config: &AdapterConfiguration<C>,*/ pid: ID) -> Result<Url> {
        self.build_uri(&format!("cms/content/{}", pid))
    }

    pub fn build_page_uri_for_pid(&self, /*config: &AdapterConfiguration<C>,*/ pid: ID) -> Result<Url> {
        self.build_uri(&format!("cms/page-tree/{}", pid))
    }

    pub fn build_uri_for_resource_type(&self, /*config: &AdapterConfiguration<C>,*/ resource_type: &str) -> Result<Url> {
        self.build_uri(&format!("{}", self.path_for_resource_type(resource_type)))
    }

    pub fn build_uri_for_resource(&self, /*config: &AdapterConfiguration<C>,*/ resource_type: &str, identifier: ID) -> Result<Url> {
        self.build_uri(&format!("{}/{}", self.path_for_resource_type(resource_type), identifier))
    }

    fn path_for_resource_type(&self, resource_type: &str) -> String {
        resource_type
            .replace('.', "-")
            .trim_start_matches('_')
            .to_lowercase()
    }
}
