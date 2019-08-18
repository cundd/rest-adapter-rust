use serde::Deserialize;
use crate::ID;

/// Struct to map to TYPO3 page records
#[derive(Debug, Deserialize)]
pub struct Page {
    id: ID,

    pid: ID,

    title: String,

    #[serde(rename = "author")]
    author: String,

    #[serde(rename = "authorEmail")]
    author_email: String,

    #[serde(rename = "modificationTime")]
    modification_time: i32,

    #[serde(rename = "creationTime")]
    creation_time: i32,

    #[serde(rename = "navigationTitle")]
    navigation_title: String,

    #[serde(rename = "seoTitle")]
    seo_title: String,

    #[serde(rename = "noIndex")]
    no_index: bool,

    #[serde(rename = "noFollow")]
    no_follow: bool,

    #[serde(rename = "ogTitle")]
    og_title: String,

    #[serde(rename = "ogDescription")]
    og_description: Option<String>,

    #[serde(rename = "twitterTitle")]
    twitter_title: String,

    #[serde(rename = "twitterDescription")]
    twitter_description: Option<String>,

//    #[serde(rename = "twitterImage")]
//    twitter_image: String,

    #[serde(rename = "canonicalLink")]
    canonical_link: String,

    #[serde(default)]
    children: Vec<Page>,
}

impl Page {
    /// Return the page UID
    pub fn id(&self) -> ID {
        self.id
    }

    /// Return the parent page identifier
    pub fn pid(&self) -> ID {
        self.pid
    }

    /// Return the title
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Return the author
    pub fn author(&self) -> &str {
        &self.author
    }

    /// Return the author's email
    pub fn author_email(&self) -> &str {
        &self.author_email
    }

    /// Return the page's creation time (`crdate`)
    pub fn creation_time(&self) -> i32 {
        self.creation_time
    }

    /// Return the page's modification time (`tstamp`)
    pub fn modification_time(&self) -> i32 {
        self.modification_time
    }

    /// Return the navigation title
    pub fn navigation_title(&self) -> &str {
        &self.navigation_title
    }

    /// Return the SEO title
    pub fn seo_title(&self) -> &str {
        &self.seo_title
    }

    /// Return the state of the no-index flag
    pub fn no_index(&self) -> bool {
        self.no_index
    }

    /// Return the state of the no-follow flag
    pub fn no_follow(&self) -> bool {
        self.no_follow
    }

    /// Return the Open Graph title
    pub fn og_title(&self) -> &str {
        &self.og_title
    }

    /// Return the Open Graph description
    pub fn og_description(&self) -> Option<&str> {
        match self.og_description {
            None => None,
            Some(ref d) => Some(d.as_str())
        }
    }

    /// Return the Twitter title
    pub fn twitter_title(&self) -> &str {
        &self.twitter_title
    }

    /// Return the Twitter description
    pub fn twitter_description(&self) -> Option<&str> {
        match self.twitter_description {
            None => None,
            Some(ref d) => Some(d.as_str())
        }
    }

//    /// Return the Twitter image
//    pub fn twitter_image(&self) -> &str {
//        &self.twitter_image
//    }

    /// Return the canonical URL
    pub fn canonical_link(&self) -> &str {
        &self.canonical_link
    }

    /// Return the page's child pages
    pub fn children(&self) -> &[Page] {
        self.children.as_slice()
    }
}

