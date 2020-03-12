use std::fs;
use std::fs::File;
use std::io::{Error, Read, Write};
use std::path::Path;
use std::path::PathBuf;

use crate::git;
use crate::renderer;

extern crate dirs;
extern crate serde;
extern crate serde_json;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Config {
    pub templates_dir: String,
    pub templates_repo: String,
}

pub fn init() -> Result<Config, Error> {
    let dir = match dirs::home_dir() {
        Some(path) => PathBuf::from(path),
        None => PathBuf::from(""),
    };

    let mut dir = dir.into_os_string().into_string().unwrap();
    dir = dir + "/.charon";

    ensure_root_dir(&dir)?;
    ensure_config_file(&dir)?;
    let config = load_config(&dir)?;

    ensure_template_dir(&config)?;
    Ok(config)
}

fn ensure_root_dir(dir: &str) -> Result<(), Error> {
    let r = fs::create_dir_all(Path::new(dir));
    match r {
        Ok(fc) => fc,
        Err(error) => return Err(error),
    };

    Ok(())
}

fn ensure_config_file(dir: &str) -> Result<(), Error> {
    let conf_path = String::from(dir) + "/config.json";
    if Path::new(&conf_path).exists() {
        return Ok(());
    }

    let default_config = get_default_config(dir);
    let new_data = serde_json::to_string(&default_config).unwrap();
    let mut dst = File::create(conf_path)?;
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
    match git::init(&config.templates_dir, &config.templates_repo) {
        Ok(()) => (),
        Err(error) => match error {
            git::GitError::InitError => println!("Init Error"),
            git::GitError::AddRemoteError => println!("Add Remote Error")
        }
    };

    match git::update(&config.templates_dir) {
        Ok(()) => (),
        Err(_e) => renderer::errors::update_templates(),
    }

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

fn get_default_config(dir: &str) -> Config {
    let template_dir = format!("{}{}", dir, "/templates");

    let config = Config { 
        templates_dir: template_dir,
        templates_repo: String::from("https://github.com/perryrh0dan/templates")
    };

    return config;
}
