use std::process::ExitCode;

mod cli;
use crate::cli::config::*;
use crate::cli::parse::*;
use crate::cli::prompt::*;

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
    match parse() {
        SubCommands::Init { directory_path } => {
            let config = unwrapOrFailure!(new_config(directory_path.to_str().unwrap().to_owned()));

            let test = setup(config.directory);

            unwrapOrFailure!(test);

            println!("done");
        }
        SubCommands::Listfiles {} => {
            let config = unwrapOrFailure!(read_config());

            let entries = unwrapOrFailure!(list_files(config.clone().directory));

            for entry in entries {
                eprintln!("{: >width$}{: >width$}", entry.0, entry.1, width = 20);
            }
        }
        SubCommands::Listtags {} => {
            let config = unwrapOrFailure!(read_config());

            let entries = unwrapOrFailure!(list_tags(config.clone().directory));

            for entry in entries {
                eprintln!("{: >width$}{: >width$}", entry.0, entry.1, width = 20);
            }
        }
        SubCommands::Getfile { tags } => {
            let config = unwrapOrFailure!(read_config());

            let files = unwrapOrFailure!(get_files(config.clone().directory, &tags));

            let file = unwrapOrFailure!(choose_file(files));

            println!("{}", file);
        }
        SubCommands::Addfile { file_path, option } => {
            let config = unwrapOrFailure!(read_config());

            // TODO actually move or the file to the directory based on `option`

            unwrapOrFailure!(add_file(
                config.clone().directory,
                &file_path.to_str().unwrap().to_owned() // TODO see if you can make this less ugly everytime
            ));
        }
        SubCommands::Addtag { names } => {
            let config = unwrapOrFailure!(read_config());

            for name in names {
                unwrapOrFailure!(add_tag(config.clone().directory, &name));
            }
        }
        SubCommands::Assign { tag, file } => {
            let config = unwrapOrFailure!(read_config());

            unwrapOrFailure!(assign(config.clone().directory, &tag, &file));
        }
        SubCommands::Removefile { names } => {
            let config = unwrapOrFailure!(read_config());

            for name in names {
                unwrapOrFailure!(delete_file(config.clone().directory, &name));
            }
        }
        SubCommands::Removetag { names } => {
            let config = unwrapOrFailure!(read_config());

            for name in names {
                unwrapOrFailure!(delete_tag(config.clone().directory, &name));
            }
        }
        SubCommands::Unassign { tag, file } => {
            let config = unwrapOrFailure!(read_config());

            unwrapOrFailure!(unassign(config.clone().directory, &tag, &file));
        }
    }

    ExitCode::SUCCESS
}
