use std::fs;
use std::fs::File;
use std::io::{Error, Read, Write};
use std::path::Path;
use std::path::PathBuf;

use crate::git;

extern crate dirs;
extern crate serde;
extern crate serde_json;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Config {
    pub templates_dir: String,
    pub template_repo: String,
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
    let config = load_config(&dir)?;

    ensure_template_dir(&config)?;
    Ok(config)
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

fn ensure_template_dir(config: &Config) -> Result<(), Error> {
    // Create if dir not exists
    let r = fs::create_dir_all(Path::new(&config.templates_dir));
    match r {
        Ok(fc) => fc,
        Err(error) => return Err(error),
    }

    // Initialize git repository
    match git::init(&config.template_repo, &config.templates_dir) {
        Ok(()) => (),
        Err(error) => match error {
            git::GitError::InitError => println!("Init Error"),
            git::GitError::AddRemoteError => println!("Add Remote Error"),
            git::GitError::UpdateError => println!("Update Error")
        }
    };

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
        template_repo: String::from("https://github.com/perryrh0dan/templates")
    };

    return config;
}
