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
use crate::tags::storage::*;

macro_rules! unwrapOrFailure {
    ($statement:expr) => {{
        let s = $statement;
        if let Err(e) = s {
            // TODO, this currently results in errors mixed from the sqlite library, and my own written error strings
            eprintln!("{}", e);
            return ExitCode::FAILURE;
        }
        s.unwrap()
    }};
}

pub fn main() -> ExitCode {
    // create config if it doesnt exist
    if !config::exists() {
        eprintln!("config file does not exist. Creating it now");
        let mut directory_path = dirs::home_dir().unwrap();
        directory_path.push("tagged");

        unwrapOrFailure!(config::new(directory_path.to_str().unwrap().to_owned()));
    }

    let config = unwrapOrFailure!(config::read());

    // create db if it doesnt exist
    if !db_exists(config.clone().directory) {
        eprintln!("Database does not exist. Creating it now");
        let config = config::read().unwrap();
        unwrapOrFailure!(setup(config.directory));
    }

    match parse::parse() {
        SubCommands::Listfiles {} => {
            let entries = unwrapOrFailure!(list_files(config.clone().directory));

            for entry in entries {
                eprintln!("{: >width$}{: >width$}", entry.0, entry.1, width = 20);
            }
        }
        SubCommands::Listtags {} => {
            let entries = unwrapOrFailure!(list_tags(config.clone().directory));

            for entry in entries {
                eprintln!("{: >width$}{: >width$}", entry.0, entry.1, width = 20);
            }
        }
        SubCommands::Getfile { tags } => {
            let files = unwrapOrFailure!(get_files(config.clone().directory, &tags));

            let file = unwrapOrFailure!(prompt::choose_file(files));

            let mut path = config.directory;
            path.push(file);
            println!("{}", path.to_str().unwrap());
        }
        SubCommands::Addfile { file_path, option } => {
            // TODO actually move or the file to the directory based on `option`

            let temp = file_path.clone();
            let file_name = temp.file_name().unwrap();
            let mut final_file_path = PathBuf::new();
            final_file_path.push(config.clone().directory);
            final_file_path.push(file_name);

            match option {
                parse::AddFileOptions::Link => {
                    dbg!(symlink_auto(file_path, final_file_path));
                }
                parse::AddFileOptions::Move => {
                    dbg!(better_copy(file_path.clone(), final_file_path));
                    dbg!(better_delete(file_path));
                }
                parse::AddFileOptions::Copy => {
                    dbg!(better_copy(file_path, final_file_path));
                }
            }

            unwrapOrFailure!(add_file(
                config.clone().directory,
                &file_name.to_str().unwrap().to_owned() // TODO see if you can make this less ugly everytime
            ));
        }
        SubCommands::Addtag { names } => {
            for name in names {
                unwrapOrFailure!(add_tag(config.clone().directory, &name));
            }
        }
        SubCommands::Assign { tag, file } => {
            unwrapOrFailure!(assign(config.clone().directory, &tag, &file));
        }
        SubCommands::Removefile { names } => {
            for name in names {
                unwrapOrFailure!(delete_file(config.clone().directory, &name));
            }
        }
        SubCommands::Removetag { names } => {
            for name in names {
                unwrapOrFailure!(delete_tag(config.clone().directory, &name));
            }
        }
        SubCommands::Unassign { tag, file } => {
            unwrapOrFailure!(unassign(config.clone().directory, &tag, &file));
        }
    }

    ExitCode::SUCCESS
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
