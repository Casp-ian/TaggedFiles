use std::path::PathBuf;

use crate::Config;
use sqlite::*;

fn db(mut path: PathBuf) -> Result<Connection> {
    path.push("info.db");
    open(path)
}

pub fn setup(config: Config) -> Result<()> {
    let conn = db(config.directory)?;
    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS tags (name TEXT);
        CREATE TABLE IF NOT EXISTS tags_files (tagid INT, fileid INT);
        CREATE TABLE IF NOT EXISTS files (name TEXT);
        ",
    )
}

// pub fn get_files(config: Config, tag_names: Vec<&str>) -> Result<()> {
//     let conn = db(config.directory)?;
//     // TODO
//     conn.execute(format!("INSERT INTO tags (name) values ('{}');", tag_name))
// }

pub fn add_tag(config: Config, tag_name: &str) -> Result<()> {
    let conn = db(config.directory)?;
    conn.execute(format!("INSERT INTO tags (name) values ('{}');", tag_name))
}

pub fn add_file(config: Config, file_name: &str) -> Result<()> {
    let conn = db(config.directory)?;
    conn.execute(format!(
        "INSERT INTO files (name) values ('{}');",
        file_name
    ))
}

pub fn assign(config: Config, tag_name: &str, file_name: &str) -> Result<()> {
    let conn = db(config.directory)?;
    conn.execute(format!(
        "
        INSERT INTO tags_files (tagid, fileid)
        VALUES (
            (SELECT rowid FROM tags WHERE name = '{}'),
            (SELECT rowid FROM files WHERE name = '{}')
        );
        ",
        tag_name, file_name,
    ))
}

// pub fn remove_tag(tag_name: &str) -> Result<()> {
//     let conn = db();
//     conn.execute("CREATE TABLE test (test INT);")
// }

// pub fn remove_file(file_name: &str) -> Result<()> {
//     let conn = db();
//     conn.execute("CREATE TABLE test (test INT);")
// }

// pub fn unassign(tag_name: &str, file_name: &str) -> Result<()> {
//     let conn = db();
//     conn.execute("CREATE TABLE test (test INT);")
// }
