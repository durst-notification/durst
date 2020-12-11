use super::*;

use serde::Deserialize;

use std::fs::File;
use std::io::BufReader;
use std::io::Read;

#[derive(Deserialize, Debug)]
pub struct DurstConfiguration {}

/// Loads the default config file, but without spitting errors if no config is there
pub fn load_config_default() -> DurstConfiguration {
    let config_basepath = var("XDG_CONFIG_HOME")
        .or_else(|_| var("HOME").map(|home| format!("{}/.config", home)))
        .unwrap();
    let config_path = format!("{}/durst/config.yml", config_basepath);

    match read_file(config_path) {
        Ok(config_str) => parse_configuration(config_str),
        Err(_) => parse_configuration("".to_string()),
    }
}

/// Loads the configuration file from a given path
pub fn load_config_path(path: String) -> DurstConfiguration {
    let config_str = read_file(path.to_string()).expect("Cannot read configuration file");
    parse_configuration(config_str)
}

/// Parse any YAML-String and spit out the corresponding configuration
pub fn parse_configuration(config_in: String) -> DurstConfiguration {
    serde_yaml::from_str(&config_in).expect("Cannot parse YAML configuration file")
}

fn read_file(path: String) -> Result<String, std::io::Error> {
    let file = File::open(path)?;
    println!("file {:?}", file);
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(contents)
}
