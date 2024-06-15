use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;
use std::process::ExitCode;
use symlink::{self, symlink_auto};

mod cli;
use crate::cli::parse::SubCommands;
use crate::cli::*;

mod tags;
use crate::tags::db;

pub fn main() -> ExitCode {
    // create config if it doesnt exist
    if !config::exists() {
        eprintln!("config file does not exist. Creating it now");
        let mut directory_path = dirs::home_dir().unwrap();
        directory_path.push("tagged");

        if let Err(e) = config::new(directory_path.to_str().unwrap().to_owned()) {
            eprintln!("{}", e);
            return ExitCode::FAILURE;
        }
    }

    let config = config::read();
    if let Err(e) = config {
        eprintln!("{}", e);
        return ExitCode::FAILURE;
    }
    let config = config.unwrap();

    // create db if it doesnt exist
    if !db::db_exists(config.clone().directory) {
        eprintln!("Database does not exist. Creating it now");
        let config = config::read().unwrap();
        if let Err(e) = db::setup(config.directory) {
            eprintln!("{}", e.to_string());
            return ExitCode::FAILURE;
        }
    }

    let result = match parse::parse() {
        SubCommands::Listfiles {} => list_files(&config),
        SubCommands::Listtags {} => list_tags(&config),
        SubCommands::Getfile { tags } => get_file(&config, tags),
        SubCommands::Addfile { file_path, option } => add_file(file_path, &config, option),
        SubCommands::Addtag { names } => add_tag(names, &config),
        SubCommands::Assign { tag, file } => assign(&config, tag, file),
        SubCommands::Removefile { names } => remove_file(names, &config),
        SubCommands::Removetag { names } => remove_tag(names, &config),
        SubCommands::Unassign { tag, file } => unassign(config, tag, file),
    };

    if let Err(e) = result {
        eprintln!("Something went wrong, {}", e);
        return ExitCode::FAILURE;
    } else {
        return ExitCode::SUCCESS;
    }
}

fn unassign(config: config::Config, tag: String, file: String) -> Result<(), String> {
    if let Err(e) = db::unassign(config.clone().directory, &tag, &file) {
        return Err(e.to_string());
    }
    Ok(())
}

fn remove_tag(names: Vec<String>, config: &config::Config) -> Result<(), String> {
    for name in names {
        if let Err(e) = db::delete_tag(config.clone().directory, &name) {
            return Err(e.to_string());
        }
    }
    Ok(())
}

fn remove_file(names: Vec<String>, config: &config::Config) -> Result<(), String> {
    for name in names {
        let mut path = config.clone().directory;
        path.push(name.clone());
        if let Err(e) = better_delete(path) {
            return Err(e.to_string());
        }
        if let Err(e) = db::delete_file(config.clone().directory, &name) {
            return Err(e.to_string());
        }
    }
    Ok(())
}

fn assign(config: &config::Config, tag: String, file: String) -> Result<(), String> {
    if let Err(e) = db::assign(config.clone().directory, &tag, &file) {
        return Err(e.to_string());
    }
    Ok(())
}

fn add_tag(names: Vec<String>, config: &config::Config) -> Result<(), String> {
    for name in names {
        if let Err(e) = db::add_tag(config.clone().directory, &name) {
            return Err(e.to_string());
        }
    }
    Ok(())
}

fn add_file(
    file_path: PathBuf,
    config: &config::Config,
    option: parse::AddFileOptions,
) -> Result<(), String> {
    let file_path = file_path.canonicalize().unwrap();

    let temp = file_path.clone();
    let file_name = temp.file_name().unwrap();
    let mut final_file_path = PathBuf::new();
    final_file_path.push(config.clone().directory);
    final_file_path.push(file_name);

    match option {
        parse::AddFileOptions::Link => {
            if let Err(e) = symlink_auto(file_path, final_file_path) {
                return Err(e.to_string());
            }
        }
        parse::AddFileOptions::Move => {
            if let Err(e) = better_copy(file_path.clone(), final_file_path) {
                return Err(e.to_string());
            }
            if let Err(e) = better_delete(file_path) {
                return Err(e.to_string());
            }
        }
        parse::AddFileOptions::Copy => {
            if let Err(e) = better_copy(file_path, final_file_path) {
                return Err(e.to_string());
            }
        }
    }

    if let Err(e) = db::add_file(
        config.clone().directory,
        &file_name.to_str().unwrap().to_owned(), // TODO see if you can make this less ugly everytime
    ) {
        return Err(e.to_string());
    }
    Ok(())
}

fn get_file(config: &config::Config, tags: Vec<String>) -> Result<(), String> {
    let files = db::get_files(config.clone().directory, &tags);
    if let Err(e) = files {
        return Err(e.to_string());
    }

    let file = prompt::choose_file(files.unwrap());
    if let Err(e) = file {
        return Err(e.to_string());
    }

    let mut path = config.directory.clone();
    path.push(file.unwrap());
    println!("{}", path.to_str().unwrap());
    Ok(())
}

fn list_tags(config: &config::Config) -> Result<(), String> {
    let entries = db::list_tags(config.clone().directory);
    if let Err(e) = entries {
        return Err(e.to_string());
    }

    for entry in entries.unwrap() {
        eprintln!("{: >width$}{: >width$}", entry.0, entry.1, width = 20);
    }
    Ok(())
}

fn list_files(config: &config::Config) -> Result<(), String> {
    let entries = db::list_files(config.clone().directory);
    if let Err(e) = entries {
        return Err(e.to_string());
    }

    for entry in entries.unwrap() {
        eprintln!("{: >width$}{: >width$}", entry.0, entry.1, width = 20);
    }
    Ok(())
}

fn better_delete(dst: impl AsRef<Path>) -> io::Result<()> {
    if dst.as_ref().is_file() {
        return fs::remove_file(dst);
    }
    fs::remove_dir_all(dst)
}

fn better_copy(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<u64> {
    if src.as_ref().is_file() {
        return fs::copy(src, dst);
    }

    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            better_copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(0)
}
