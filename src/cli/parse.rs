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
    /// list all files and what tags they have
    Listfiles,

    /// list all tags and what files have them
    Listtags,

    /// search file based on tags and opens a prompt if multiple files match, prints absolute path to stdout
    Getfile {
        /// tags to search on, if none given will return all files
        tags: Option<Vec<String>>,

        /// Instead of asking which specific file you want, it will just spit them all out seperated by spaces
        #[arg(long, default_value_t = false)]
        multiple: bool,
    },

    /// add a new file to the tagged files
    Addfile {
        /// path of the file to add
        #[arg(required = true)]
        file_path: PathBuf,

        /// how the file will be added to the tagged files
        #[arg(short, long, value_enum, default_value_t = AddFileOptions::None)]
        option: AddFileOptions,
    },

    /// add a tag
    Addtag {
        // TODO add option to add special tags here when added
        /// all tag names to add
        #[arg(required = true)]
        names: Vec<String>,
    },

    /// assign a tag to a file
    Assign {
        /// tag to be assigned to the file
        #[arg(required = true)]
        tag: String,

        /// file that a tag should be assigned to
        #[arg(required = true)]
        file: String,
    },

    /// remove a file
    Removefile {
        /// all file names to be removed
        #[arg(required = true)]
        names: Vec<String>,
    },

    /// remove a tag, also removes tag from all files
    Removetag {
        /// all tag names to be removed
        #[arg(required = true)]
        names: Vec<String>,
    },

    /// unasign a tag from a file
    Unassign {
        /// tag that should be unnasigned to the file
        tag: String,

        /// file that should be unnasigned to the tag
        file: String,
    },

    // TODO
    /// add a file to the database that is in the tagged directory
    AddUnstoredFile,

    // TODO
    /// add a file to the tagged directory that is in the database
    AddUnmarkedFile,

    // TODO maybe relinking for if a file gets moved/renamed, tho this could be a suggestion for the addUnstore and Unmarked

    // TODO this name sucks
    /// symlink the files with these tags to a directory, handles sub tags as directories
    GetAsLinkDirectory {
        /// tags to search on, if none given will return all files
        tags: Option<Vec<String>>,
    },
}

#[derive(ValueEnum, Clone)]
pub enum AddFileOptions {
    None, // TODO
    Move,
}

pub fn parse() -> SubCommands {
    let cli = Cli::parse();

    // TODO check of paths are valid or other validation that clap cant do

    cli.command
}
