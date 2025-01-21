use ron::ser::{to_string_pretty, PrettyConfig};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::ops::Index;
use std::path::PathBuf;

use super::types::{ChildTag, FileTagConnection, StoredFile, Tag, TagFilter};

#[derive(Debug, Deserialize, Serialize)]
struct Data {
    files: Vec<StoredFile>,
    connections: Vec<FileTagConnection>,
    tags: Vec<Tag>,
    child_tags: Vec<ChildTag>,
}

pub struct Database {
    data: Data,
    location: PathBuf,
}
impl Database {
    fn get_or_create_file(path: &PathBuf) -> Result<File, String> {
        let file_result = File::open(path);
        let file: File;
        if let Err(e) = file_result {
            if e.kind() != std::io::ErrorKind::NotFound {
                return Err(e.to_string());
            }

            eprintln!("Creating a new database file at {:?}", &path); // TODO i dont really want printlines littered around

            Database::create_file(path)?;

            let file_result = File::open(path);
            if let Err(e) = file_result {
                return Err(e.to_string());
            }
            file = file_result.unwrap();
        } else {
            file = file_result.unwrap();
        }

        return Ok(file);
    }

    fn create_file(path: &PathBuf) -> Result<(), String> {
        let file_result = File::create(path);
        if let Err(e) = file_result {
            return Err(e.to_string());
        }
        let mut file = file_result.unwrap();

        let default_data = to_string_pretty(
            &Data {
                files: vec![],
                connections: vec![],
                tags: vec![],
                child_tags: vec![],
            },
            PrettyConfig::new().indentor("  ".to_owned()),
        )
        .unwrap();

        if let Err(e) = file.write_all(default_data.as_bytes()) {
            return Err(e.to_string());
        }

        // it is no use returning this file because it is in write-only mode
        return Ok(());
    }

    fn apply(self) -> Result<(), String> {
        let file_result = File::create(&self.location); //create really is just open in write-only mode
        if let Err(e) = file_result {
            return Err(e.to_string());
        }
        let mut file = file_result.unwrap();

        let new_data =
            to_string_pretty(&self.data, PrettyConfig::new().indentor("  ".to_owned())).unwrap();

        if let Err(e) = file.write_all(new_data.as_bytes()) {
            return Err(e.to_string());
        }
        return Ok(());
    }

    pub fn open(mut path: PathBuf) -> Result<Database, String> {
        path.push("data.ron"); // TODO this should come from config
        let file = Database::get_or_create_file(&path)?;

        let mut output = String::new();
        let mut buf_reader = BufReader::new(file);
        if let Err(e) = buf_reader.read_to_string(&mut output) {
            return Err(e.to_string());
        }
        let data: Data = ron::from_str(&output).unwrap();

        return Ok(Database {
            data,
            location: path,
        });
    }

    pub fn list_files(self) -> Result<Vec<StoredFile>, String> {
        return Ok(self.data.files);
    }

    // TODO update for child tags
    pub fn list_tags(self) -> Result<Vec<Tag>, String> {
        return Ok(self.data.tags);
    }

    pub fn get_files(self, filter: TagFilter) -> Result<Vec<StoredFile>, String> {
        let mut result = self.data.files.clone();
        if filter.is_empty() {
            return Ok(result);
        }

        let mut connections = self.data.connections.clone();

        connections.retain(|x| filter.allowed_tags.contains(&x.tag_name));
        connections.retain(|x| !filter.denied_tags.contains(&x.tag_name));

        let file_names: Vec<String> = connections.iter().map(|x| x.file_name.clone()).collect();
        result.retain(|x| file_names.contains(&x.name));

        return Ok(result);
    }

    pub fn add_tag(mut self, name: String) -> Result<(), String> {
        let new_tag = Tag {
            name,
            children: vec![],
        };
        self.data.tags.push(new_tag);

        self.apply()?;
        return Ok(());
    }

    pub fn add_file(mut self, name: String, path: String, last_used: u64) -> Result<(), String> {
        let new_tag = StoredFile {
            name,
            path,
            last_used,
        };
        self.data.files.push(new_tag);

        self.apply()?;
        return Ok(());
    }

    /// adds allowed tags, and removes denied tags from the given file
    pub fn set_tags(mut self, file_name: String, mut tag_filter: TagFilter) -> Result<(), String> {
        let mut removable_tags: Vec<String> = vec![];
        for (i, tag) in tag_filter.allowed_tags.iter_mut().enumerate() {
            if self.data.tags.iter().filter(|x| x.name == *tag).count() == 0 {
                // TODO should offer to create it as well
                eprintln!("the tag '{}' does not exist, skipping it", tag);
            }
        }

        for tag in tag_filter.allowed_tags {
            // TODO dont do duplicates
            self.data.connections.push(FileTagConnection {
                file_name: file_name.clone(),
                tag_name: tag,
            });
        }

        for tag in tag_filter.denied_tags {
            self.data
                .connections
                .retain(|x| !(x.file_name == file_name && x.tag_name == tag));
        }

        // let existing = &self.conn.connections[&ron::Value::String(file_name)];

        // let test: Result<Vec<String>, ron::Error> = existing.to_owned().into_rust();
        // self.conn.connections.insert(file_name);
        self.apply()?;
        return Ok(());
    }

    pub fn delete_tag(mut self, tag_name: String) -> Result<(), String> {
        // TODO remove connections
        let index = self.data.tags.iter().position(|x| x.name == tag_name);
        if let Some(i) = index {
            self.data.tags.remove(i);
            return Ok(());
        } else {
            return Err(format!("Couldnt find tag with name: {}", tag_name));
        }
    }

    pub fn delete_file(mut self, file_name: String) -> Result<(), String> {
        // TODO remove connections
        let index = self.data.files.iter().position(|x| x.name == file_name);
        if let Some(i) = index {
            self.data.files.remove(i);
            return Ok(());
        } else {
            return Err(format!("Couldnt find file with name: {}", file_name));
        }
    }
}
