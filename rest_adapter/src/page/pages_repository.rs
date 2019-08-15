use super::Page;
use crate::prelude_internal::*;
use crate::uri_builder::UriBuilder;
use crate::request_helper::fetch;

pub struct PageRepository<'a> {
    uri_builder: &'a UriBuilder,
}

impl<'a> PageRepository<'a> {
    pub fn new(uri_builder: &'a UriBuilder) -> Self {
        PageRepository { uri_builder }
    }

    pub fn find_by_start_pid(&self, pid: ID) -> Result<Page> {
        fetch(self.uri_builder.build_page_uri_for_pid(pid)?)
    }
}
