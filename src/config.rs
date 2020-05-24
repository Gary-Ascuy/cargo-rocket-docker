use toml;
use serde_derive::{Serialize, Deserialize};
use std::fs::File;
use std::io::Read;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    package: Package,
    docker: Option<Docker>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Package {
    name: String,
    version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Docker {
    name: Option<String>,
    version: Option<String>,
    image: Option<String>,
    maintainer: Option<String>,
    packages: Option<Packages>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Packages {
    build: Option<String>,
    image: Option<String>,
}

pub fn parse() -> Config {
    let mut file = File::open("./Cargo.toml").expect("Could not load docker configuration from Cargo.toml");
    let mut string = String::new();
    file.read_to_string(&mut string).expect("Could not load docker configuration from Cargo.toml");

    let config: Config = toml::from_str(&string).expect("Could not parse with required parameters.");

    //let config = Loader::from_file("./Cargo.toml").unwrap(); //.expect("as"); //.expect("Could not load docker configuration from Cargo.toml");
    // let config: Config = toml::from_str(r#"
    //     name = 'server'
    //     version = '666.0.0'
    //     [docker]
    //     name = 'server'
    //     version = '1.0.0'
    //     maintainer = 'fromfile<file@from.com>'
    //     [docker.packages]
    //     build = "asd"
    //     image = "xxxxx"
    // "#).expect("asdsad");

    // println!("aaa {}", config.version);

    config
}
