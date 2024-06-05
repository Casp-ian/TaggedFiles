use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    // TODO add debug or verbose option or whatever here
    #[command(subcommand)]
    command: SubCommands,
}

#[derive(Subcommand)]
pub enum SubCommands {
    // TODO add value_name to most of these
    /// initialize the vault for tagged files
    Init {
        /// the directory where all tagged files will be stored
        directory_path: PathBuf,
    },

    /// search file based on tags and opens a prompt if multiple files match, prints absolute path to stdout
    Getfile {
        /// tags to search on, if none given will return all files
        tags: Vec<String>,
    },

    /// add a new file to the tagged files
    Addfile {
        /// path of the file to add
        #[arg(short, long)]
        file_path: PathBuf,

        /// how the file will be added to the tagged files
        #[arg(short, long, value_enum, default_value_t = AddFileOptions::Link)]
        option: AddFileOptions,
    },

    /// add a tag
    Addtag {
        // TODO add option to add special tags here when added
        /// all tag names to add
        #[clap(required = true)]
        names: Vec<String>,
    },

    /// assign a tag to a file
    Assign {
        /// file that a tag should be assigned to
        file: String,

        /// tag to be assigned to the file
        tag: String,
    },

    /// remove a file
    Removefile {
        /// all file names to be removed
        #[clap(required = true)]
        names: Vec<String>,
    },

    /// remove a tag, also removes tag from all files
    Removetag {
        /// all tag names to be removed
        #[clap(required = true)]
        names: Vec<String>,
    },

    /// unasign a tag from a file
    Unassign {
        /// file that should be unnasigned to the tag
        file: String,

        /// tag that should be unnasigned to the file
        tag: String,
    },
}

#[derive(ValueEnum, Clone)]
pub enum AddFileOptions {
    Link,
    Move,
    Copy,
}

pub fn parse() -> SubCommands {
    let cli = Cli::parse();

    // TODO check of paths are valid

    cli.command
}
