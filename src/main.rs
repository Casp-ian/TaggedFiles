use std::process::ExitCode;

mod cli;
use crate::cli::config::*;
use crate::cli::parse::*;
use crate::cli::prompt::*;

mod tags;
use crate::tags::storage::*;

macro_rules! unwrapOrFailure {
    ($statement:expr) => {{
        if let Err(e) = $statement {
            eprintln!("{}", e);
            return ExitCode::FAILURE;
        }
        $statement.unwrap()
    }};
}

pub fn main() -> ExitCode {
    match parse() {
        SubCommands::Init { directory_path } => {
            let config = unwrapOrFailure!(new_config(directory_path.to_str().unwrap().to_owned()));

            let test = setup(config);

            unwrapOrFailure!(test);

            println!("done");
        }
        SubCommands::Getfile { tags } => {
            // let config = unwrapOrFailure!(read_config());

            // println!("{:?}", tags);

            println!("/home/caspar/tagged");
        }
        SubCommands::Addfile { file_path, option } => {
            todo!();
        }
        SubCommands::Addtag { names } => {
            todo!();
        }
        SubCommands::Assign { file, tag } => {
            todo!();
        }
        SubCommands::Removefile { names } => {
            todo!();
        }
        SubCommands::Removetag { names } => {
            todo!();
        }
        SubCommands::Unassign { file, tag } => {
            todo!();
        }
    }

    ExitCode::SUCCESS
}
