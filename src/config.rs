use std::path::Path;
use std::io::{Error, Read, Write};
use std::path::PathBuf;
use std::fs::File;
use serde::{Serialize, Deserialize};

extern crate dirs;
extern crate serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub template_dir: String,
    pub email: String,
}

pub fn init() -> Result<Config, Error> {
    let dir = match dirs::home_dir() {
        Some(path) => PathBuf::from(path),
        None => PathBuf::from(""),
    };

    ensure_config_file(&dir)?;
    get(&dir)
}

fn ensure_config_file(dir: &PathBuf)-> Result<(), Error> {
    if Path::new(dir).exists() {
        return Ok(());
    }

    let defaultconfig = Config{ template_dir: String::from("test"), email: String::from("test") };
    let new_data = serde_json::to_string(&defaultconfig).unwrap();
    let mut dst = File::create(dir)?;
    dst.write(new_data.as_bytes())?;

    Ok(())
}

fn ensure_init_dir() {

}

fn format_init_dir(path: &str) {
       
}

fn get(dir: &PathBuf) -> Result<Config, Error> {
    // Open file
  let mut src = File::open(Path::new(dir))?;
  let mut data = String::new();

  // Write to data string
  src.read_to_string(&mut data)?;
  let config: Config = serde_json::from_str(&data).unwrap();
  return Ok(config);
}
