mod http_client_trait;
mod reqwest_client;

pub use http_client_trait::HttpClientTrait;
use crate::http_client::reqwest_client::ReqwestClient;

pub type DefaultClient = ReqwestClient;

pub fn default_client() -> DefaultClient {
    reqwest_client::ReqwestClient::new()
}
