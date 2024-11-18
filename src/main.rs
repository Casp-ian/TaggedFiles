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
    let config = match config::read() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("{}", e);
            return ExitCode::FAILURE;
        }
    };

    // create db if it doesnt exist
    if !db::db_exists(config.clone().tag_directory) {
        eprintln!("Database does not exist. Creating it now");
        let config = config::read().unwrap();
        if let Err(e) = db::setup(config.tag_directory) {
            eprintln!("{}", e.to_string());
            return ExitCode::FAILURE;
        }
    }

    let result = match parse::parse() {
        SubCommands::Listfiles {} => list_files(&config),
        SubCommands::Listtags {} => list_tags(&config),
        SubCommands::Getfile { tags, multiple } => get_file_path(&config, tags, multiple),
        SubCommands::Addfile { file_path, option } => add_file(file_path, &config, option),
        SubCommands::Addtag { names } => add_tag(names, &config),
        SubCommands::Assign { tag, file } => assign(&config, tag, file),
        SubCommands::Removefile { names } => remove_file(names, &config),
        SubCommands::Removetag { names } => remove_tag(names, &config),
        SubCommands::Unassign { tag, file } => unassign(&config, tag, file),
        SubCommands::GetAsLinkDirectory { tags } => get_as_link_directory(&config, tags),
        _ => Err("not yet implemented".to_owned()), // TODO
    };

    if let Err(e) = result {
        eprintln!("Something went wrong, {}", e);
        return ExitCode::FAILURE;
    } else {
        return ExitCode::SUCCESS;
    }
}

fn unassign(config: &config::Config, tag: String, file: String) -> Result<(), String> {
    if let Err(e) = db::unassign(config.clone().tag_directory, &tag, &file) {
        return Err(e.to_string());
    }
    Ok(())
}

fn remove_tag(names: Vec<String>, config: &config::Config) -> Result<(), String> {
    for name in names {
        if let Err(e) = db::delete_tag(config.clone().tag_directory, &name) {
            return Err(e.to_string());
        }
    }
    Ok(())
}

fn remove_file(names: Vec<String>, config: &config::Config) -> Result<(), String> {
    for name in names {
        if let Err(e) = db::delete_file(config.clone().tag_directory, &name) {
            return Err(e.to_string());
        }
    }
    Ok(())
}

fn assign(config: &config::Config, tag: String, file: String) -> Result<(), String> {
    if let Err(e) = db::assign(config.clone().tag_directory, &tag, &file) {
        return Err(e.to_string());
    }
    Ok(())
}

fn add_tag(names: Vec<String>, config: &config::Config) -> Result<(), String> {
    for name in names {
        if let Err(e) = db::add_tag(config.clone().tag_directory, &name) {
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
    let mut file_path = file_path.canonicalize().unwrap();

    let temp = file_path.clone();
    let file_name = temp.file_name().unwrap();

    match option {
        parse::AddFileOptions::None => {
            // i think this should be empty
        }
        parse::AddFileOptions::Move => {
            let mut final_file_path = PathBuf::new();
            final_file_path.push(config.clone().tag_directory);
            final_file_path.push(file_name);

            if let Err(e) = better_copy(file_path.clone(), final_file_path.clone()) {
                return Err(e.to_string());
            }
            file_path = final_file_path; //change filepath so it actually changes in the database as this path
            if let Err(e) = better_delete(file_path.clone()) {
                return Err(e.to_string());
            }
        }
    }

    if let Err(e) = db::add_file(
        config.clone().tag_directory,
        &file_name.to_str().unwrap().to_owned(), // TODO see if you can make this less ugly everytime
        &file_path.to_str().unwrap().to_owned(),
    ) {
        return Err(e.to_string());
    }
    Ok(())
}

fn get_file_path(
    config: &config::Config,
    tags: Option<Vec<String>>,
    multiple: bool,
) -> Result<(), String> {
    let tags = match tags {
        Some(list) => list,
        None => vec![],
    };

    let files = db::get_files(config.clone().tag_directory, &tags);
    if let Err(e) = files {
        return Err(e.to_string());
    }
    let actual_files = files.unwrap();

    if actual_files.is_empty() {
        println!("."); // return . so  if used in a script, it doesnt cd to home
        return Err("no files match".to_owned());
    }
    if multiple {
        for file in actual_files {
            println!("{}", file.path);
            // this keeps a trailing space, dont think it will be a problem
        }
        Ok(())
    } else {
        let file = prompt::choose_file(actual_files);
        if let Err(e) = file {
            println!("."); // return . so  if used in a script, it doesnt cd to home
            return Err(e.to_string());
        }

        println!("{}", file.unwrap().path);
        Ok(())
    }
}

fn list_tags(config: &config::Config) -> Result<(), String> {
    let entries = db::list_tags(config.clone().tag_directory);
    if let Err(e) = entries {
        return Err(e.to_string());
    }

    for entry in entries.unwrap() {
        eprintln!("{: >width$}{: >width$}", entry.0, entry.1, width = 20);
    }
    Ok(())
}

fn list_files(config: &config::Config) -> Result<(), String> {
    let entries = db::list_files(config.clone().tag_directory);
    if let Err(e) = entries {
        return Err(e.to_string());
    }

    // TODO
    for entry in entries.unwrap() {
        eprintln!(
            "{: >smallWidth$}{: >width$}{: >smallWidth$}",
            entry.0,
            entry.1,
            entry.2,
            width = 40,
            smallWidth = 20,
        );
    }
    Ok(())
}

fn get_as_link_directory(config: &config::Config, tags: Option<Vec<String>>) -> Result<(), String> {
    let tags = match tags {
        Some(list) => list,
        None => vec![],
    };

    let files = db::get_files(config.clone().tag_directory, &tags);
    if let Err(message) = files {
        return Err(message.to_string());
    }

    // create and clear directory
    let mut final_directory_path = PathBuf::new();
    final_directory_path.push(config.clone().tag_directory);
    final_directory_path.push(config.clone().link_directory_name);

    if final_directory_path.exists() {
        if let Err(message) = fs::remove_dir_all(final_directory_path.clone()) {
            return Err(message.to_string());
        }
    }
    if let Err(message) = fs::create_dir(final_directory_path) {
        return Err(message.to_string());
    }

    // do thing
    let files = match files {
        Ok(files) => files,
        Err(message) => return Err(message.to_string()),
    };
    for file in files {
        let mut final_file_path = PathBuf::new();
        final_file_path.push(config.clone().tag_directory);
        final_file_path.push(config.clone().link_directory_name);
        final_file_path.push(file.name);
        if let Err(message) = symlink_auto(file.path, final_file_path) {
            return Err(message.to_string());
        }
    }

    return Ok(());
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
