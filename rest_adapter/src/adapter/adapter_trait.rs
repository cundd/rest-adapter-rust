use crate::error::*;
use crate::ID;
use serde::de::DeserializeOwned;

/// Trait for Adapter implementations
///
/// [`RestAdapter`] provides the default implementation
pub trait AdapterTrait {
    /// Fetch all records of the given `resource_type`
    ///
    /// See also [https://rest.cundd.net/FAQ/#resource-type]
    fn find_all<T>(&self, resource_type: &str) -> Result<Vec<T>, Error> where T: DeserializeOwned;

    /// Fetch the record with the given `identifier` and `resource_type`
    ///
    /// See also [https://rest.cundd.net/FAQ/#resource-type]
    fn find_by_identifier<T>(&self, resource_type: &str, identifier: ID) -> Result<T, Error> where T: DeserializeOwned;
}
