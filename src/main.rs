use std::process::ExitCode;

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
    if config::exists() {
        eprintln!("config file does not exit. Creating it now");
        let mut directory_path = dirs::home_dir().unwrap();
        directory_path.push("tagged");

        unwrapOrFailure!(config::new(directory_path.to_str().unwrap().to_owned()));
    }

    let config = unwrapOrFailure!(config::read());

    // create db if it doesnt exist
    if db_exists(config.clone().directory) {
        eprintln!("Database does not exit. Creating it now");
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

            println!("{}", file);
        }
        SubCommands::Addfile { file_path, option } => {
            // TODO actually move or the file to the directory based on `option`

            unwrapOrFailure!(add_file(
                config.clone().directory,
                &file_path.to_str().unwrap().to_owned() // TODO see if you can make this less ugly everytime
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
