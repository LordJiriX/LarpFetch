// Here we load / process larpfatch.json;

use serde::{Deserialize, Serialize};
use simply_colored::*;
use std::{
    fs::{File, create_dir},
    io::{self, Write},
    path::{Path, PathBuf},
};

use crate::{Config, TargetOS};

#[derive(Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct ConfigFile {
    path: PathBuf,
    main: bool,
}
impl ConfigFile {
    pub fn new(path: PathBuf, main: bool) -> Self {
        Self { path, main }
    }
    pub fn main(&self) -> bool {
        self.main
    }
    pub fn path(&self) -> &PathBuf {
        &self.path
    }
}
#[derive(Clone, PartialEq, PartialOrd, Deserialize, Serialize, Debug)]
pub enum ConfigError {
    ConfigFileNotFound,
    MissingConfigOption(String),
    UnknownConfigOption(String),
}
impl ConfigError {
    pub fn print(&self) {
        match self {
            ConfigError::ConfigFileNotFound => {
                println!("{RED}Error can't find config file\n Use larpfatch --make_config");
            }
            ConfigError::MissingConfigOption(option) => {
                println!("{RED}Error unknown config option: {option}");
            }
            ConfigError::UnknownConfigOption(option) => {
                println!("{RED}Error can't find {option} config option");
            }
        }
    }
}
pub fn config_file_location(os: &TargetOS) -> Option<PathBuf> {
    match os {
        TargetOS::Windows => {
            return Some(PathBuf::from("config.json"));
        }
        TargetOS::Linux => {
            std::env::home_dir().map(|home| home.join(".config/larpfetch/config.json"))
        }
        TargetOS::MacOS => {
            return None;
        }
        TargetOS::Unknown => {
            return None;
        }
    }
}
pub fn make_config(path: &PathBuf) -> io::Result<()> {
    let path_s = path.to_str().unwrap().replace("/config.json", "");
    let path_d = Path::new(&path_s);
    if !path_d.exists() {
        let _ = create_dir(path_d).unwrap();
    }

    if path.exists() {
        println!("Waring: Overriding config!");
    }
    let mut file = File::create(path).unwrap();
    let config = Config::default();
    let json = serde_json::to_string_pretty(&config).unwrap();
    file.write_all(json.as_bytes())?;
    Ok(())
}
