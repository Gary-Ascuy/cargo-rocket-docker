use toml;
use serde_derive::{Serialize, Deserialize};
use std::clone::Clone;

use std::fs::File;
use std::io::Read;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub package: Package,
    pub docker: Option<Docker>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Package {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Docker {
    pub name: Option<String>,
    pub version: Option<String>,
    pub image: Option<String>,
    pub maintainer: Option<String>,
    pub tag: Option<String>,
    pub custom_tags: Option<Vec<String>>,
    pub packages: Option<Packages>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Packages {
    pub build: Option<String>,
    pub image: Option<String>,
}

pub fn parse() -> Config {
    let mut file = File::open("./Cargo.toml").expect("Could not load docker configuration from Cargo.toml");
    let mut string = String::new();
    file.read_to_string(&mut string).expect("Could not load docker configuration from Cargo.toml");

    let config: Config = toml::from_str(&string).expect("Could not parse with required parameters.");
    config
}
