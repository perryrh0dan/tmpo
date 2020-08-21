use std::fs;
use std::fs::File;
use std::io::{Error, Read, Write};
use std::path::Path;
use std::path::PathBuf;

use crate::git;

extern crate dirs;
extern crate serde;
extern crate serde_yaml;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Config {
  pub auto_update: bool,
  pub templates_dir: String,
  pub templates_repositories: Vec<RepositoryOptions>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct RepositoryOptions {
  pub name: String,
  pub description: String,
  pub git_options: git::GitOptions,
}

impl Config {
  pub fn get_repository_config(&self, name: &str) -> Option<RepositoryOptions> {
    let config = self.templates_repositories.iter().find(|&x| x.name == name);

    if config.is_some() {
      return Some(config.unwrap().clone());
    } else {
      return None;
    }
  }

  pub fn save(&self) -> Result<(), Error>{
    save_config(self)
  }
}

pub fn init() -> Result<Config, Error> {
  ensure_root_dir()?;
  ensure_config_file()?;

  let config = load_config()?;

  ensure_template_dir(&config.templates_dir)?;

  Ok(config)
}

fn ensure_root_dir() -> Result<(), Error> {
  let dir = directory();
  let r = fs::create_dir_all(Path::new(&dir));
  match r {
    Ok(fc) => fc,
    Err(error) => return Err(error),
  };

  Ok(())
}

fn ensure_template_dir(dir: &str) -> Result<(), Error> {
  let r = fs::create_dir_all(Path::new(dir));
  match r {
    Ok(fc) => fc,
    Err(error) => return Err(error),
  };

  Ok(())
}

fn ensure_config_file() -> Result<(), Error> {
  let conf_path = config_location();
  if Path::new(&conf_path).exists() {
    return Ok(());
  }

  let default_config = get_default_config();
  save_config(&default_config)?;

  Ok(())
}

fn save_config(config: &Config) -> Result<(), Error> {
  let path = config_location();

  let new_data = serde_yaml::to_string(config).unwrap();
  let mut dst = File::create(path)?;
  dst.write(new_data.as_bytes())?;

  Ok(())
}

fn load_config() -> Result<Config, Error> {
  let path = config_location();
  // Open file
  let mut src = File::open(path)?;
  let mut data = String::new();

  // Write to data string
  src.read_to_string(&mut data)?;
  let config: Config = serde_yaml::from_str(&data).unwrap();
  return Ok(config);
}

fn get_default_config() -> Config {
  let dir = directory();
  let template_dir = format!("{}{}", dir.to_string_lossy(), "/templates");
  let mut repo_options = Vec::<RepositoryOptions>::new();

  let git_options = git::GitOptions {
    enabled: true,
    url: Some(String::from("https://github.com/perryrh0dan/templates")),
    auth: Some(String::from("none")),
    token: None,
    username: None,
    password: None,
  };

  repo_options.push(RepositoryOptions {
    name: String::from("Default"),
    description: String::from("Default template repository from tpoe"),
    git_options: git_options,
  });

  let config = Config {
    auto_update: true,
    templates_dir: template_dir,
    templates_repositories: repo_options,
  };

  return config;
}

fn config_location() -> PathBuf {
  let mut dir = match dirs::home_dir() {
    Some(path) => PathBuf::from(path),
    None => PathBuf::from(""),
  };

  dir.push(&PathBuf::from(".tmpo/config.yaml"));

  return dir;
}

pub fn directory() -> PathBuf {
  let mut path = config_location();
  path.pop();

  return path;
}
