use serde::Deserialize;
use super::ID;

/// Struct to map to TYPO3 content elements
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
    /// Return the content element UID
    pub fn id(&self) -> ID {
        self.id
    }

    /// Return the content element's page identifier
    pub fn pid(&self) -> ID {
        self.pid
    }

    /// Return the content element's creation time (`crdate`)
    pub fn creation_time(&self) -> i32 {
        self.creation_time
    }

    /// Return the content element's modification time (`tstamp`)
    pub fn modification_time(&self) -> i32 {
        self.modification_time
    }

    /// Return the rendered HTML of the content element
    pub fn content(&self) -> &str {
        &self.content
    }
}
