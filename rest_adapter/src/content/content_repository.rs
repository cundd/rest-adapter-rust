use super::Content;
use crate::prelude_internal::*;
use crate::uri_builder::UriBuilder;
use crate::request_helper::fetch;

pub struct ContentRepository<'a> {
    uri_builder: &'a UriBuilder,
}

impl<'a> ContentRepository<'a> {
    pub fn new(uri_builder: &'a UriBuilder) -> Self {
        ContentRepository { uri_builder }
    }

    pub fn find_by_pid(&self, pid: ID) -> Result<Vec<Content>> {
        fetch(self.uri_builder.build_content_uri_for_pid(pid)?)
    }
}
