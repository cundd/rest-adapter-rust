use super::uri_builder::UriBuilder;
use super::error::Result;
use super::ID;

pub trait RepositoryTrait<T> {
    fn find_by_pid(&self, pid: ID) -> Result<Vec<T>>;
}
