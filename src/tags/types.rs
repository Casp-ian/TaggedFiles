use serde::{Deserialize, Serialize};
use std::fmt::Display;

// TODO add marker for files outside and inside of the special directory

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct StoredFile {
    pub name: String,
    pub path: String,
    pub last_used: u64, // epoch
}
impl Display for StoredFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

// TODO add marker for files special and autodetect tags
// also maybe colors :)
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Tag {
    pub name: String,
    pub children: Vec<ChildTag>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ChildTag {
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FileTagConnection {
    pub file_name: String,
    pub tag_name: String,
}

#[derive(Debug, Clone)]
pub struct TagFilter {
    pub allowed_tags: Vec<String>,
    pub denied_tags: Vec<String>,
    pub allowed_child_tags: Vec<(String, String)>,
    pub denied_child_tags: Vec<(String, String)>,
}

impl TagFilter {
    pub fn is_empty(&self) -> bool {
        return self.allowed_tags.is_empty()
            && self.denied_tags.is_empty()
            && self.allowed_child_tags.is_empty()
            && self.denied_child_tags.is_empty();
    }
}
