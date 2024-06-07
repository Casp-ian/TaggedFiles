use std::path::PathBuf;

use rusqlite::Connection;

// TODO, sql injection risk, pretty much in every method
fn db(mut path: PathBuf) -> rusqlite::Result<Connection> {
    path.push("info.db");

    Connection::open(path)
}

pub fn setup(db_dir: PathBuf) -> rusqlite::Result<usize> {
    let conn = db(db_dir)?;
    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS tags (name TEXT);
        CREATE TABLE IF NOT EXISTS tags_files (tagid INT, fileid INT);
        CREATE TABLE IF NOT EXISTS files (name TEXT);
        ",
        (),
    )
}

pub fn get_files(db_dir: PathBuf, tag_names: &Vec<String>) -> rusqlite::Result<usize> {
    let conn = db(db_dir)?;

    // TODO, test, and make it return file names instead of tags
    conn.execute(
        "SELECT * FROM tags WHERE name in (?1);",
        [tag_names.join(", ")],
    )
}

pub fn add_tag(db_dir: PathBuf, tag_name: &String) -> rusqlite::Result<usize> {
    let conn = db(db_dir)?;
    conn.execute("INSERT INTO tags (name) values (?1);", [tag_name])
}

pub fn add_file(db_dir: PathBuf, file_name: &String) -> rusqlite::Result<usize> {
    let conn = db(db_dir)?;
    conn.execute("INSERT INTO files (name) values (?1);", [file_name])
}

pub fn assign(db_dir: PathBuf, tag_name: &String, file_name: &String) -> rusqlite::Result<usize> {
    let conn = db(db_dir)?;
    conn.execute(
        "
        INSERT INTO tags_files (tagid, fileid)
        VALUES (
            (SELECT rowid FROM tags WHERE name = ?1),
            (SELECT rowid FROM files WHERE name = ?2)
        );
        ",
        [tag_name, file_name],
    )
}

pub fn delete_tag(db_dir: PathBuf, tag_name: &String) -> rusqlite::Result<usize> {
    let conn = db(db_dir)?;
    conn.execute("DELETE FROM tags WHERE name = ?1;", [tag_name])
}

pub fn delete_file(db_dir: PathBuf, file_name: &String) -> rusqlite::Result<usize> {
    let conn = db(db_dir)?;
    conn.execute("DELETE FROM files WHERE name = ?1;", [file_name])
}

pub fn unassign(db_dir: PathBuf, tag_name: &String, file_name: &String) -> rusqlite::Result<usize> {
    let conn = db(db_dir)?;
    conn.execute(
        "
        DELETE FROM tags_files 
        WHERE tagid = (SELECT rowid FROM tags WHERE name = ?1)
        AND fileid = (SELECT rowid FROM files WHERE name = ?2);
        ",
        [tag_name, file_name],
    )
}
