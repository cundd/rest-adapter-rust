mod content_repository;

use serde::Deserialize;
use super::ID;
pub use content_repository::ContentRepository;

#[derive(Debug, Deserialize)]
pub struct Content {
    id: ID,

    #[serde(rename = "pid")]
    pid: ID,

    #[serde(rename = "creationTime")]
    creation_time: i32,

    #[serde(rename = "modificationTime")]
    modification_time: i32,

    content: String,
}

impl Content {
    pub fn id(&self) -> ID {
        self.id
    }
    pub fn pid(&self) -> ID {
        self.pid
    }
    pub fn creation_time(&self) -> i32 {
        self.creation_time
    }
    pub fn modification_time(&self) -> i32 {
        self.modification_time
    }
    pub fn content(&self) -> &str {
        &self.content
    }
}
