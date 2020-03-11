use std::fs;
use std::fs::File;
use std::io::{Error, Read, Write};
use serde::{Serialize, Deserialize};
use std::path::Path;
use std::path::PathBuf;

extern crate dirs;
extern crate serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub templates_dir: String,
    pub email: String,
}

pub fn init() -> Result<Config, Error> {
    let dir = match dirs::home_dir() {
        Some(path) => PathBuf::from(path),
        None => PathBuf::from(""),
    };

    let mut dir = dir.into_os_string().into_string().unwrap();
    dir = dir + "/.init";

    ensure_init_dir(&dir)?;
    ensure_config_file(&dir)?;
    ensure_template_dir(&dir)?;

    load_config(&dir)
}

fn ensure_config_file(dir: &str) -> Result<(), Error> {
    let dir = String::from(dir) + "/config.json";
    if Path::new(&dir).exists() {
        return Ok(());
    }

    let default_config = get_default_config();
    let new_data = serde_json::to_string(&default_config).unwrap();
    let mut dst = File::create(dir)?;
    dst.write(new_data.as_bytes())?;

    Ok(())
}

fn ensure_template_dir(dir: &str) -> Result<(), Error> {
    let dir = String::from(dir) + "/templates";
    // Create if dir not exists
    let r = fs::create_dir_all(Path::new(&dir));
    match r {
        Ok(fc) => fc,
        Err(error) => return Err(error),
    }

    Ok(())
}

fn ensure_init_dir(dir: &str) -> Result<(), Error> {
    let r = fs::create_dir_all(Path::new(dir));
    match r {
        Ok(fc) => fc,
        Err(error) => return Err(error),
    };

    Ok(())
}

fn load_config(dir: &str) -> Result<Config, Error> {
  let dir = String::from(dir) + "/config.json";
    // Open file
  let mut src = File::open(Path::new(&dir))?;
  let mut data = String::new();

  // Write to data string
  src.read_to_string(&mut data)?;
  let config: Config = serde_json::from_str(&data).unwrap();
  return Ok(config);
}

fn get_default_config() -> Config {
    let config = Config { 
        templates_dir: String::from("/home/thomas/dev/init/templates/default"), 
        email: String::from("test") 
    };

    return config;
}
