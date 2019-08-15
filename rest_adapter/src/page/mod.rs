mod pages_repository;

use serde::Deserialize;
pub use pages_repository::PageRepository;
use crate::ID;

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
    pub fn id(&self) -> ID {
        self.id
    }

    pub fn pid(&self) -> ID {
        self.pid
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn author(&self) -> &str {
        &self.author
    }

    pub fn author_email(&self) -> &str {
        &self.author_email
    }
    pub fn creation_time(&self) -> i32 {
        self.creation_time
    }

    pub fn modification_time(&self) -> i32 {
        self.modification_time
    }

    pub fn navigation_title(&self) -> &str {
        &self.navigation_title
    }

    pub fn seo_title(&self) -> &str {
        &self.seo_title
    }

    pub fn no_index(&self) -> bool {
        self.no_index
    }

    pub fn no_follow(&self) -> bool {
        self.no_follow
    }

    pub fn og_title(&self) -> &str {
        &self.og_title
    }

    pub fn og_description(&self) -> Option<&str> {
        match self.og_description {
            None => None,
            Some(ref d) => Some(d.as_str())
        }
    }

    pub fn twitter_title(&self) -> &str {
        &self.twitter_title
    }

    pub fn twitter_description(&self) -> Option<&str> {
        match self.twitter_description {
            None => None,
            Some(ref d) => Some(d.as_str())
        }
    }

//    pub fn twitter_image(&self) -> &str {
//        &self.twitter_image
//    }

    pub fn canonical_link(&self) -> &str {
        &self.canonical_link
    }

    pub fn children(&self) -> &[Page] {
        self.children.as_slice()
    }
}

