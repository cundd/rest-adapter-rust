extern crate serde;
extern crate serde_json;
extern crate log;

mod error;
mod id;
mod uri_builder;
mod adapter_configuration;
mod adapter;
mod page;
mod content;
mod repository;
mod factory;
mod http_client;
mod prelude_internal;

//use uri_builder::UriBuilder;

pub use prelude_internal::*;
pub use content::Content;
pub use page::Page;
pub use factory::Factory;
pub use adapter::*;
pub use http_client::HttpClientTrait;
pub use url::Url;
