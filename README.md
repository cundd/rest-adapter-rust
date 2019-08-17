# Rust Adapter for [Cundd Rest](https://rest.corn.rest)

```rust
use rest_adapter::*;
use serde::Deserialize;

#[derive(Deserialize)]
struct Person {
    name: String
}

fn run() -> Result<Vec<Person>> {
    let config = AdapterConfiguration::from_url("http://base.url.tld/rest/")?;
    let rd = RestAdapter::new(config);

    rd.find_all::<Person>("Vendor-RealEstate-Person")
}
```
