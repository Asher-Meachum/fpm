use std::env;
use std::fmt;
use std::fs::{self, File};
use std::io::Read;
use std::path::{Path, PathBuf};

use serde::Serialize;

use crate::types::{self, Link};

#[derive(serde::Deserialize, Serialize, Debug)]
pub struct Config {
    path: PathBuf,
    links: Vec<Link>,
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Loaded from {}:", self.path.display())?;
        self.links.iter().fold(Ok(()), |result, link| {
            result.and_then(|_| writeln!(f, "{}", link))
        })
    }
}

impl Config {
    pub fn init(custom_config_dir: Option<String>) -> Result<Config, types::Error> {
        let mut config_path = match custom_config_dir {
            Some(p) => PathBuf::from(p),
            None => {
                match env::home_dir() {
                    Some(b) => b,
                    None => return Err(types::Error::Fs),
                }
            },
        };

        config_path.push(".fpm.toml");

        if !Path::new(&config_path).is_file() {
            File::create(&config_path)?;
            let config = Config {
                path: config_path,
                links: Vec::new(),
            };

            Ok(config)
        } else {
            let mut raw_config = String::new();
            File::open(config_path)?.read_to_string(&mut raw_config)?;
            let config = toml::from_str(raw_config.as_str())?;
            Ok(config)
        }
    }

    pub fn add_link(&mut self, link: Link) {
        self.links.push(link);
    }

    pub fn remove_link(&mut self, name: impl ToString) {
        self.links.retain(|x| x.name() != name.to_string());
    }

    pub fn links(&self) -> &Vec<Link> {
        &self.links
    }

    pub fn save(&mut self) -> Result<(), types::Error> {
        let toml = toml::to_string(self)?;
        fs::write(self.path.clone(), toml)?;

        Ok(())
    }
}
