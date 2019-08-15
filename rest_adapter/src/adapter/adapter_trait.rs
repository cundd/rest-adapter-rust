use crate::error::*;
use crate::ID;
use serde::de::DeserializeOwned;

pub trait AdapterTrait {
    /// Fetch all records of the given `resource_type`
    ///
    /// See also [https://rest.cundd.net/FAQ/#resource-type]
    fn find_all<T: DeserializeOwned>(&self, resource_type: &str) -> Result<Vec<T>>;

    /// Fetch the record with the given `identifier` and `resource_type`
    ///
    /// See also [https://rest.cundd.net/FAQ/#resource-type]
    fn find_by_identifier<T: DeserializeOwned>(&self, resource_type: &str, identifier: ID) -> Result<T>;
}
