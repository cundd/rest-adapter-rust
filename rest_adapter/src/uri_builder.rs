use reqwest::Url;
use crate::prelude_internal::*;

pub struct UriBuilder {
    config: AdapterConfiguration
}

impl UriBuilder {
    pub fn new(config: AdapterConfiguration) -> Self {
        UriBuilder { config }
    }

    pub fn build_uri(&self, path: &str) -> Result<Url> {
        Ok(self.config.base_uri().join(&format!("{}/", path))?)
    }

    pub fn build_content_uri_for_pid(&self, pid: ID) -> Result<Url> {
        self.build_uri(&format!("cms/content/{}", pid))
    }

    pub fn build_page_uri_for_pid(&self, pid: ID) -> Result<Url> {
        self.build_uri(&format!("cms/page-tree/{}", pid))
    }

    pub fn build_uri_for_resource_type(&self, resource_type: &str) -> Result<Url> {
        self.build_uri(&format!("{}", self.path_for_resource_type(resource_type)))
    }

    pub fn build_uri_for_resource(&self, resource_type: &str, identifier: ID) -> Result<Url> {
        self.build_uri(&format!("{}/{}", self.path_for_resource_type(resource_type), identifier))
    }

    fn path_for_resource_type(&self, resource_type: &str) -> String {
        resource_type
            .replace('.', "-")
            .trim_start_matches('_')
            .to_lowercase()
    }
}
