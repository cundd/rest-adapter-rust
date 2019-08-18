mod http_client_trait;
mod reqwest_client;
mod fixture_client;

pub use http_client_trait::HttpClientTrait;
pub use fixture_client::FixtureClient;
use crate::http_client::reqwest_client::ReqwestClient;

pub type DefaultClient = ReqwestClient;

pub fn default_client() -> DefaultClient {
    reqwest_client::ReqwestClient::new()
}
