extern crate serde;
extern crate toml;

use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub school: String,
    pub access_token: Option<String>,
    pub temp: Option<Temp>,
}

#[derive(Serialize, Deserialize)]
pub struct Temp {
    pub auth_code: String,
}

impl<'a> Config {
    pub fn parse_from_file(filename: &'a str) -> Result<Self, String> {
        let mut file = match File::open(filename) {
            Ok(f) => f,
            Err(e) => return Err(e.description().to_owned()),
        };
        let mut contents = String::new();
        match file.read_to_string(&mut contents) {
            Ok(_) => (),
            Err(e) => return Err(e.description().to_owned()),
        };

        let config: Config = match toml::from_str(contents.as_str()) {
            Ok(c) => c,
            Err(e) => return Err(e.description().to_owned()),
        };
        Ok(config)
    }

    pub fn write_config(&self, filename: &'a str) -> Result<(), String> {
        let config_toml = match toml::to_string(self) {
            Ok(c) => c,
            Err(e) => return Err(e.description().to_owned()),
        };

        let mut file = match File::create(filename) {
            Ok(f) => f,
            Err(e) => return Err(e.description().to_owned()),
        };

        match file.write_all(config_toml.as_bytes()) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.description().to_owned()),
        }
    }
}
