use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;
use std::process::ExitCode;
use symlink::{self, symlink_auto};
use tags::db::Database;
use tags::tag_relations;

mod cli;
use crate::cli::parse::SubCommands;
use crate::cli::*;

mod tags;

pub fn main() -> ExitCode {
    // create config if it doesnt exist
    let config = match config::read() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("{}", e);
            return ExitCode::FAILURE;
        }
    };

    let result = match parse::parse() {
        SubCommands::Listfiles {} => list_files(&config),
        SubCommands::Listtags {} => list_tags(&config),
        SubCommands::Getfile { tags, multiple } => get_file_path(&config, tags, multiple),
        SubCommands::Addfile { file_path, option } => add_file(file_path, &config, option),
        SubCommands::Addtag { names } => add_tag(names, &config),
        SubCommands::Settags { tags, file } => set_tags(&config, tags, file),
        SubCommands::Removefile { names } => remove_file(names, &config),
        SubCommands::Removetag { names } => remove_tag(names, &config),
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

fn remove_tag(names: Vec<String>, config: &config::Config) -> Result<(), String> {
    for name in names {
        if let Err(e) = Database::open(config.clone().managed_directory)?.delete_tag(name) {
            return Err(e.to_string());
        }
    }
    Ok(())
}

fn remove_file(names: Vec<String>, config: &config::Config) -> Result<(), String> {
    for name in names {
        if let Err(e) = Database::open(config.clone().managed_directory)?.delete_file(name) {
            return Err(e.to_string());
        }
    }
    Ok(())
}

fn set_tags(config: &config::Config, tags: Vec<String>, file: String) -> Result<(), String> {
    let filter = tag_relations::parse_tags(&tags)?;
    if let Err(e) = Database::open(config.clone().managed_directory)?.set_tags(file, filter) {
        return Err(e.to_string());
    }
    Ok(())
}

fn add_tag(names: Vec<String>, config: &config::Config) -> Result<(), String> {
    for name in names {
        // TODO idk if validation should be here, should be moved to parse
        if name.contains("/") || name.contains("+") || name.contains("-") || name.contains(" ") {
            return Err("tag names cannot contain '/', '+', '-' or whitespace".to_owned());
        }
        if let Err(e) = Database::open(config.clone().managed_directory)?.add_tag(name) {
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
            final_file_path.push(config.clone().managed_directory);
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

    if let Err(e) = Database::open(config.clone().managed_directory)?.add_file(
        file_name.to_str().unwrap().to_owned(), // TODO see if you can make this less ugly everytime
        file_path.to_str().unwrap().to_owned(),
        0, // TODO date epoch
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

    // TODO actually use these tags
    let filter = tag_relations::parse_tags(&tags)?;

    let files = Database::open(config.clone().managed_directory)?.get_files(filter);
    if let Err(e) = files {
        return Err(e.to_string());
    }
    let actual_files = files.unwrap();

    if actual_files.is_empty() {
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
            return Err(e.to_string());
        }

        println!("{}", file.unwrap().path);
        Ok(())
    }
}

fn list_tags(config: &config::Config) -> Result<(), String> {
    let entries = Database::open(config.clone().managed_directory)?.list_tags()?;

    for entry in entries {
        eprintln!("{:?}", entry);
    }
    Ok(())
}

fn list_files(config: &config::Config) -> Result<(), String> {
    let entries = Database::open(config.clone().managed_directory)?.list_files()?;

    // TODO
    for entry in entries {
        eprintln!("{}", entry);
    }
    Ok(())
}

fn get_as_link_directory(config: &config::Config, tags: Option<Vec<String>>) -> Result<(), String> {
    let tags = match tags {
        Some(list) => list,
        None => vec![],
    };

    let filter = tag_relations::parse_tags(&tags)?;

    let files = Database::open(config.clone().managed_directory)?.get_files(filter);
    if let Err(message) = files {
        return Err(message.to_string());
    }

    // create and clear directory
    let mut final_directory_path = PathBuf::new();
    final_directory_path.push(config.clone().managed_directory);
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
        final_file_path.push(config.clone().managed_directory);
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
