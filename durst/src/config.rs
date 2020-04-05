use super::*;

use serde::Deserialize;

use std::fs::File;
use std::io::BufReader;
use std::io::Read;

#[derive(Deserialize, Debug)]
pub struct Rule {
    pub test: String,
    pub maybe: Option<String>,
}

pub fn load_config(path: String) -> Vec<Rule> {
    match read_file(path) {
        Ok(config_str) => {
            let config: Rule = serde_yaml::from_str(&config_str).unwrap();
            debug!("{:?}", config);
            Vec::new()
        }
        Err(err) => {
            error!("Error reading config file: {:?}", err.to_string());
            Vec::new()
        }
    }
}

fn read_file(path: String) -> Result<String, std::io::Error> {
    let file = File::open(path)?;
    println!("file {:?}", file);
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(contents)
}
