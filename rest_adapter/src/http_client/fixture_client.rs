use crate::{HttpClientTrait, Result, Url};

/// A HTTP Client implementation that will always return the data from the instance
///
/// This can be used to mock HTTP clients
#[derive(Clone)]
pub struct FixtureClient {
    data: String
}

impl FixtureClient {
    /// Return a new instance that will return `data` when [`fetch`] is called
    pub fn with_data(&self, data: String) -> Self {
        Self { data }
    }
}

impl HttpClientTrait for FixtureClient {
    fn new() -> Self {
        Self { data: "".to_owned() }
    }

    fn fetch(&self, _: &Url) -> Result<String> {
        Ok(self.data.clone())
    }
}
