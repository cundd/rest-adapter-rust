extern crate serde;
extern crate serde_json;

mod error;
mod id;
mod uri_builder;
mod adapter_configuration;
mod adapter;
mod page;
mod content;
mod factory;
mod request_helper;
mod prelude_internal;

use uri_builder::UriBuilder;

pub use prelude_internal::*;
pub use content::Content;
pub use page::Page;
pub use factory::Factory;
pub use adapter::*;

pub fn main() {
    println!("Hello from lib!");

    main2().unwrap();
}


fn main2() -> Result<()> {
    let config = AdapterConfiguration::from_url("http://localhost:8888/")?;
    let builder = UriBuilder::new(config);

    let new_post: Vec<Content> = content::ContentRepository::new(&builder).find_by_pid(1)?;

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
