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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
  pub repositories_dir: PathBuf,
  pub templates_dir: PathBuf,
  pub template_repositories: Vec<RepositoryOptions>,
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

impl Config {
  pub fn get_repositories(&self) -> Vec<String> {
    let mut repositories = self.get_custom_repositories();

    // Used for single repository templates
    repositories.push(String::from("templates"));

    return repositories;
  }

  pub fn get_custom_repositories(&self) -> Vec<String> {
    let mut repositories = Vec::<String>::new();

    for entry in self.template_repositories.iter() {
      repositories.push(utils::lowercase(&entry.name));
    }

    return repositories;
  }

  pub fn get_local_repositories(&self) -> Vec<String> {
    let mut repositories = Vec::<String>::new();

    for entry in self.template_repositories.iter() {
      if !entry.git_options.enabled {
        repositories.push(utils::lowercase(&entry.name))
      }
    }

    return repositories;
  }

  pub fn get_templates(&self) -> Vec<String> {
    let mut templates = Vec::<String>::new();

    for template in self.templates.iter() {
      templates.push(utils::lowercase(&template.name))
    }

    return templates;
  }

  pub fn get_repository_config(&self, name: &str) -> Option<RepositoryOptions> {
    let config = self
      .template_repositories
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

  match ensure_repositories_dir(&config.repositories_dir) {
    Ok(()) => (),
    Err(error) => {
      return Err(RunError::IO(error));
    }
  };

  match ensure_templates_dir(&config.repositories_dir) {
    Ok(()) => (),
    Err(error) => {
      return Err(RunError::IO(error));
    }
  };

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

fn ensure_repositories_dir(dir: &PathBuf) -> Result<(), Error> {
  let r = fs::create_dir_all(dir);
  match r {
    Ok(fc) => fc,
    Err(error) => return Err(error),
  };

  Ok(())
}

fn ensure_templates_dir(dir: &PathBuf) -> Result<(), Error> {
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
  for rep_option in &mut config.template_repositories {
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

fn get_default_config() -> Config {
  let dir = directory();
  let repositories_dir = dir.join("/repositories");
  let templates_dir = dir.join("/templates");
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
    repositories_dir: repositories_dir,
    templates_dir: templates_dir,
    template_repositories: repo_options,
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
