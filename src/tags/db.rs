use std::fmt::Display;
use std::fs;
use std::path::PathBuf;

use rusqlite::Connection;

// TODO add marker for files outside and inside of the special directory
// TODO last used
pub struct File {
    pub name: String,
    pub path: String,
}
impl Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

// TODO add marker for files special and autodetect tags
pub struct Tag {
    pub name: String,
    pub children: Vec<Tag>,
}

// TODO move the string "info.db" to some better position
pub fn db_exists(mut path: PathBuf) -> bool {
    path.push("info.db");
    path.exists()
}

// TODO, sql injection risk, pretty much in every method
fn db(mut path: PathBuf) -> rusqlite::Result<Connection> {
    path.push("info.db");

    Connection::open(path)
}

pub fn setup(db_dir: PathBuf) -> rusqlite::Result<()> {
    // TODO is good error handling nececary here?
    let _ = fs::create_dir(&db_dir);
    let conn = db(db_dir)?;
    // TODO add marker for files outside and inside of the special directory
    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS files (
            name TEXT NOT NULL,
            path TEXT NOT NULL,
            lastUsed INT,
            UNIQUE(name)
        );
        ",
        (),
    )?;
    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS tags_files (
            tagid INT NOT NULL, 
            fileid INT NOT NULL,
            UNIQUE(tagid, fileid)
        );
        ",
        (),
    )?;
    // TODO add marker for files special and autodetect tags
    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS tags (
            name TEXT NOT NULL,
            parentTag INT,
            UNIQUE(name)
        );
        ",
        (),
    )?;
    Ok(())
}

pub fn list_files(db_dir: PathBuf) -> rusqlite::Result<Vec<(String, String, String)>> {
    let conn = db(db_dir)?;

    // it looks like there are cases where right and full outer joins are not enabled in sqlite, so avoid using those :)
    let mut statement = conn.prepare(
        "
        SELECT f.name, f.path, COALESCE(t.name, '-no tags-') FROM files f
        LEFT JOIN tags_files j on f.rowid = j.fileid
        LEFT JOIN tags t ON t.rowid = j.tagid
        WHERE f.name IS NOT NULL
        ORDER BY f.name;
        ",
    )?;
    let mut rows = statement.query([])?;

    let mut names = Vec::<(String, String, String)>::new();
    while let Some(row) = rows.next()? {
        names.push((row.get(0)?, row.get(1)?, row.get(2)?));
    }

    Ok(names)
}

pub fn list_tags(db_dir: PathBuf) -> rusqlite::Result<Vec<(String, String)>> {
    let conn = db(db_dir)?;

    // TODO, one of these outer joins probably dont need to be
    let mut statement = conn.prepare(
        "
        SELECT t.name, COALESCE(f.name, '-no files-') FROM tags t
        LEFT JOIN tags_files j on t.rowid = j.tagid
        LEFT JOIN files f ON f.rowid = j.fileid
        WHERE t.name IS NOT NULL
        ORDER BY t.name;
        ",
    )?;
    let mut rows = statement.query([])?;

    let mut names = Vec::<(String, String)>::new();
    while let Some(row) = rows.next()? {
        names.push((row.get(0)?, row.get(1)?));
    }

    Ok(names)
}

pub fn get_files(db_dir: PathBuf, tag_names: &Vec<String>) -> rusqlite::Result<Vec<File>> {
    // if there are no tags, all files should be returned
    if tag_names.is_empty() {
        return get_all_files(db_dir);
    }

    let conn = db(db_dir)?;

    let mut statement = conn.prepare(
        "
        SELECT f.name, f.path FROM files f
        INNER JOIN tags_files j on f.rowid = j.fileid
        INNER JOIN tags t ON t.rowid = j.tagid
        WHERE t.name in (?1);
        ",
    )?;
    let mut rows = statement.query([tag_names.join(", ")])?;

    let mut files = Vec::<File>::new();
    while let Some(row) = rows.next()? {
        files.push(File {
            name: row.get(0)?,
            path: row.get(1)?,
        });
    }

    Ok(files)
}

pub fn get_all_files(db_dir: PathBuf) -> rusqlite::Result<Vec<File>> {
    let conn = db(db_dir)?;

    let mut statement = conn.prepare(
        "
        SELECT f.name, f.path FROM files f
        ",
    )?;
    let mut rows = statement.query([])?;

    let mut files = Vec::<File>::new();
    while let Some(row) = rows.next()? {
        files.push(File {
            name: row.get(0)?,
            path: row.get(1)?,
        });
    }

    Ok(files)
}

pub fn add_tag(db_dir: PathBuf, tag_name: &String) -> rusqlite::Result<usize> {
    let conn = db(db_dir)?;
    conn.execute("INSERT INTO tags (name) values (?1);", [tag_name])
}

pub fn add_file(
    db_dir: PathBuf,
    file_name: &String,
    file_path: &String,
) -> rusqlite::Result<usize> {
    let conn = db(db_dir)?;
    conn.execute(
        "INSERT INTO files (name, path) values (?1, ?2);",
        [file_name, file_path],
    )
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
