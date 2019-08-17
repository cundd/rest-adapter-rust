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

use uri_builder::UriBuilder;

pub use prelude_internal::*;
pub use content::Content;
pub use page::Page;
pub use factory::Factory;
pub use adapter::*;
pub use http_client::HttpClientTrait;
pub use url::Url;
use crate::repository::Repository;

pub fn main() {
    println!("Hello from lib!");

    main2().unwrap();
}


fn main2() -> Result<()> {
    let config = AdapterConfiguration::from_url("http://localhost:8888/")?;
    let builder = UriBuilder::new(config.clone());

    let new_post: Vec<Content> = Repository::new(&config, &builder).find_by_pid(1)?;

    println!("{:#?}", new_post);
    // Post {
    //     id: Some(
    //         101
    //     ),
    //     title: "Reqwest.rs",
    //     body: "https://docs.rs/reqwest",
    //     user_id: 1
    // }
    Ok(())
}
