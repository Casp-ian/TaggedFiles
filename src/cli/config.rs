use serde::Deserialize;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::path::PathBuf;

// TODO move this to ron, for consistency accross the code, and because ron can make this prettier
#[derive(Clone)]
pub struct Config {
    pub managed_directory: PathBuf,
    pub link_directory_name: PathBuf,
}

#[derive(Deserialize)]
struct WeakConfig {
    managed_directory: Option<PathBuf>,
    link_directory_name: Option<PathBuf>,
}

fn config_path() -> PathBuf {
    let mut config_dir = dirs::config_dir().unwrap();
    config_dir.push("tagged.toml"); // maybe i could make this configurable, but it seems like a pain for something nobody wants
    config_dir
}

pub fn exists() -> bool {
    config_path().exists()
}

pub fn read() -> Result<Config, String> {
    if !exists() {
        eprintln!("config file does not exist. Creating it now");
        if let Err(e) = create_empty_config() {
            return Err(e);
        }
    }

    let file_result = File::open(config_path());
    if file_result.is_err() {
        return Err("Couldnt open file".to_owned());
    }
    let file = file_result.unwrap();

    let mut output = String::new();
    let mut buf_reader = BufReader::new(file);
    if buf_reader.read_to_string(&mut output).is_err() {
        return Err("Couldnt read file".to_owned());
    }

    let toml: WeakConfig = toml::from_str(&output).unwrap();

    let mut tag_directory = PathBuf::new();
    let mut link_directory_name = PathBuf::new();

    if let Some(result) = toml.managed_directory {
        tag_directory.push(result);
    } else {
        // TODO this could break windows, idk if home dir is even wanted behaviour in windows
        tag_directory.push(dirs::home_dir().unwrap());
        tag_directory.push("tagged".to_owned());
    }

    if let Some(result) = toml.link_directory_name {
        link_directory_name.push(result);
    } else {
        link_directory_name.push("!link".to_owned());
    }

    return Ok(Config {
        managed_directory: tag_directory,
        link_directory_name,
    });
}

pub fn create_empty_config() -> Result<(), String> {
    let config_path = config_path();
    if exists() {
        return Err("config already exists".to_owned());
    }

    // TODO the current way of doing defaults is basically hidden to the normal user,
    // fix this by appending the defaults for unset values to the config

    // managed_directory: Option<PathBuf>,
    // link_directory_name: Option<PathBuf>,
    let default_config =
        "# welcome to the config file :), here are the default values\n# managed_directory = \"~/tagged\"\n# link_directory_name = \"!link\"".as_bytes();

    let file = File::create(config_path);
    if let Ok(mut new_file) = file {
        if new_file.write_all(default_config).is_err() {
            return Err("couldnt write to config file".to_owned());
        }
    } else if file.is_err() {
        return Err("could not create config file".to_owned());
    };
    return Ok(());
}
