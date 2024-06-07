use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::path::{Path, PathBuf};
use toml::Table;

#[derive(Clone)]
pub struct Config {
    pub directory: PathBuf,
}

pub fn read_config() -> Result<Config, String> {
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

    let toml = output.parse::<Table>().unwrap();

    let mut directory = PathBuf::new();

    directory.push(toml["directory"].as_str().unwrap().to_owned());

    return Ok(Config {
        directory: directory,
    });
}

pub fn new_config(tagged_dir: String) -> Result<Config, String> {
    let config_path = config_path();
    if !Path::new(&config_path).is_file() {
        return Err("config already exists".to_owned());
    }

    let file = File::create(config_path);
    if let Ok(mut new_file) = file {
        if new_file
            .write_all(format!("directory = '{}'", tagged_dir).as_bytes())
            .is_err()
        {
            return Err("couldnt write to config file".to_owned());
        }
    } else if file.is_err() {
        return Err("could not create config file".to_owned());
    }

    read_config()
}

pub fn config_path() -> PathBuf {
    let mut config_dir = dirs::config_dir().unwrap();
    config_dir.push("tagged.toml");
    config_dir
}
