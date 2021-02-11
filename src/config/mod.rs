use std::fs;
use std::fs::File;
use std::io::{Error, Read, Write};
use std::path::Path;
use std::path::PathBuf;

use crate::error::RunError;
use crate::git;
use crate::utils;

extern crate dirs;
extern crate serde;
use serde::{Deserialize, Serialize};
extern crate serde_yaml;
extern crate clap;
use clap::crate_version;
extern crate semver;
use semver::Version;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
  pub version: String,
  pub repositories_dir: PathBuf,
  pub templates_dir: PathBuf,
  #[serde(alias = "repositories", alias = "template_repositories", skip_serializing_if = "Vec::is_empty")]
  pub repositories: Vec<RepositoryOptions>,
  #[serde(skip_serializing_if = "Vec::is_empty", default)]
  pub templates: Vec<TemplateOptions>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct RepositoryOptions {
  pub name: String,
  pub description: Option<String>,
  pub git_options: git::Options,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct TemplateOptions {
  pub name: String,
  pub description: Option<String>,
  pub git_options: git::Options,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ConfigVersion {
  pub version: Option<String>,
}

impl Config {
  pub fn get_repository_names(&self) -> Vec<String> {
    let mut repositories = self.get_custom_repository_names();

    // Used for single repository templates
    repositories.push(String::from("templates"));

    return repositories;
  }

  pub fn get_custom_repository_names(&self) -> Vec<String> {
    let mut repositories = Vec::<String>::new();

    for entry in self.repositories.iter() {
      repositories.push(utils::lowercase(&entry.name));
    }

    return repositories;
  }

  pub fn get_repository_config(&self, name: &str) -> Option<RepositoryOptions> {
    let config = self
      .repositories
      .iter()
      .find(|&x| utils::lowercase(&x.name) == utils::lowercase(&name));

    if config.is_some() {
      return Some(config.unwrap().clone());
    } else {
      return None;
    }
  }

  pub fn save(&self) -> Result<(), Error> {
    save_config(self)
  }

  pub fn validate(&self) -> Result<(), RunError> {
    // Check repository names for reserved names
    for repository in self.get_custom_repository_names() {
      if repository  == String::from("templates") {
        return Err(RunError::Config(format!("Reserved repository name {} used", repository)));
      }
    }

    Ok(())
  }
}

pub fn init() -> Result<Config, RunError> {
  match ensure_root_dir() {
    Ok(()) => (),
    Err(error) => {
      return Err(RunError::IO(error));
    }
  };
  match ensure_config_file() {
    Ok(()) => (),
    Err(error) => {
      return Err(RunError::IO(error));
    }
  };

  let config = load_config()?;

  config.validate()?;

  // Ensure repository directory
  match ensure_dir(&config.repositories_dir) {
    Ok(()) => (),
    Err(error) => {
      return Err(RunError::IO(error));
    }
  };

  // Ensure template directory
  match ensure_dir(&config.repositories_dir) {
    Ok(()) => (),
    Err(error) => {
      return Err(RunError::IO(error));
    }
  };

  // Ensure temp directory
  match ensure_dir(&temp_dir()) {
    Ok(()) => (),
    Err(error) => {
      return Err(RunError::IO(error));
    }
  }

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

fn ensure_dir(dir: &PathBuf) -> Result<(), Error> {
  let r = fs::create_dir_all(dir);
  match r {
    Ok(fc) => fc,
    Err(error) => return Err(error),
  }

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

fn load_config() -> Result<Config, RunError> {
  let path = config_location();
  // Open file
  let mut src = File::open(path)?;
  let mut data = String::new();

  // Write to data string
  src.read_to_string(&mut data)?;
  let mut config: Config = match serde_yaml::from_str(&data) {
    Ok(data) => data,
    Err(error) => {
      log::error!("{}", error);
      return Err(RunError::Config(String::from("Deserialization")));
    }
  };

  // Need to solve config change when switching from 1.5.1 => 1.5.2
  let mut changed = false;
  for rep_option in &mut config.repositories {
    if rep_option.git_options.provider.is_none() {
      rep_option.git_options.provider = Some(git::Provider::GITHUB);
      changed = true;
    }
  }

  if changed {
    match config.save() {
      Ok(()) => (),
      Err(error) => {
        log::error!("{}", error);
      }
    };
  }

  return Ok(config);
}

pub fn get_default_config() -> Config {
  let dir = directory();
  let repositories_dir = dir.join("repositories");
  let templates_dir = dir.join("templates");
  let mut repo_options = Vec::<RepositoryOptions>::new();
  let template_options = Vec::<TemplateOptions>::new();

  let git_options = git::Options {
    enabled: true,
    provider: Some(git::Provider::GITHUB),
    url: Some(String::from("https://github.com/perryrh0dan/templates")),
    branch: Some(String::from("master")),
    auth: Some(git::AuthType::NONE),
    token: None,
    username: None,
    password: None,
  };

  repo_options.push(RepositoryOptions {
    name: String::from("Default"),
    description: Some(String::from("Default template repository from tpoe")),
    git_options: git_options,
  });

  let config = Config {
    version: String::from(crate_version!()),
    repositories_dir: repositories_dir,
    templates_dir: templates_dir,
    repositories: repo_options,
    templates: template_options,
  };

  return config;
}

pub fn config_location() -> PathBuf {
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

pub fn temp_dir() -> PathBuf {
  let path = directory().join("temp");

  return path;
}

pub fn version() -> Result<Version, RunError> {
  let path = config_location();
  // Open file
  let mut src = File::open(path)?;
  let mut data = String::new();

  // Write to data string
  src.read_to_string(&mut data)?;
  let config: ConfigVersion = match serde_yaml::from_str(&data) {
    Ok(data) => data,
    Err(error) => {
      log::error!("{}", error);
      return Err(RunError::Config(String::from("Deserialization")));
    }
  };

  if config.version.is_some() {
    let version = Version::parse(&config.version.unwrap()).unwrap();
    Ok(version)
  } else {
    let version = Version::parse("1.8.3").unwrap();
    Ok(version)
  }
}
