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
    pub fn with_data(data: String) -> Self {
        Self { data }
    }

    /// Set the `data` that will be returned when [`fetch`] is called
    pub fn set_data(mut self, data: String) -> Self {
        self.data = data;

        self
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
