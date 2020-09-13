use std::fs;
use std::fs::File;
use std::io::{Error, Write};
use std::path::{Path, PathBuf};

use crate::config::{Config, RepositoryOptions};
use crate::error::RunError;
use crate::git;
use crate::meta;
use crate::template;
use crate::utils;

#[derive(Debug)]
pub struct Repository {
  pub config: RepositoryOptions,
  pub directory: PathBuf,
  pub meta: meta::Meta,
  pub templates: Vec<template::Template>,
}

impl Repository {
  pub fn new(config: &Config, name: &str) -> Result<Repository, RunError> {
    let cfg = match config.get_repository_config(name) {
      Option::Some(cfg) => cfg,
      Option::None => {
        return Err(RunError::Repository(String::from("Not found")));
      }
    };

    let directory = Path::new(&config.template_dir).join(&utils::lowercase(name));

    // Load meta
    let meta = match meta::load_meta(&directory) {
      Ok(meta) => meta,
      Err(error) => {
        log::error!("{}", error);
        eprintln!("{}", error);
        return Err(RunError::Repository(String::from("Unable to load meta")));
      }
    };

    let mut repository = Repository {
      config: cfg,
      directory: directory,
      meta: meta,
      templates: Vec::<template::Template>::new(),
    };

    match repository.ensure_repository_dir() {
      Ok(()) => (),
      Err(error) => {
        log::error!("{}", error);
        return Err(RunError::Repository(String::from("Initialization")))
      }
    };

    // ensure git setup if enabled
    if repository.config.git_options.enabled {
      match repository.ensure_repository_git() {
        Ok(()) => (),
        Err(_) => (),
      };
    }

    repository.load_templates();

    return Ok(repository);
  }

  pub fn test(self) -> Result<(), RunError> {
    // ensure git setup if enabled
    if self.config.git_options.enabled {
      match self.ensure_repository_git() {
        Ok(()) => (),
        Err(error) => {
          log::error!("{}", error);
          return Err(RunError::Repository(String::from("Initialization")))
        },
      };
    }

    Ok(())
  }

  /// delete
  pub fn delete_repository(&self) -> Result<(), RunError> {
    log::info!(
      "Delete repository directory {}",
      &self.directory.to_owned().to_str().unwrap()
    );
    match fs::remove_dir_all(&self.directory) {
      Ok(()) => (),
      Err(error) => {
        return Err(RunError::IO(error));
      }
    }

    return Ok(());
  }

  /// Create a new template with given name in the repository directory
  pub fn create_template(&self, name: &str) -> Result<std::path::PathBuf, Error> {
    let repository_path = Path::new(&self.directory);
    let template_path = repository_path.join(&name);

    // Create template directory
    fs::create_dir(&template_path)?;

    // Create meta.json
    let meta_path = template_path.join("meta.json");
    let mut meta_file = File::create(meta_path)?;

    // Create meta data
    let mut meta = meta::default();
    meta.kind = String::from("template");
    meta.name = name.to_owned();
    meta.version = Some(String::from("1.0.0"));

    let meta_data = serde_json::to_string_pretty(&meta).unwrap();
    meta_file.write(meta_data.as_bytes())?;

    return Ok(template_path);
  }

  /// Return list of all template names in this repository
  pub fn get_templates(&self) -> Vec<String> {
    let mut templates = Vec::<String>::new();

    for template in &self.templates {
      templates.push(utils::lowercase(&template.name));
    }

    return templates;
  }

  /// Return template with given name
  pub fn get_template_by_name(&self, name: &str) -> Result<&template::Template, RunError> {
    for template in &self.templates {
      if template.name == *name {
        return Ok(template);
      }
    }

    return Err(RunError::Template(String::from("Not found")));
  }

  fn ensure_repository_dir(&self) -> Result<(), Error> {
    if !self.directory.exists() {
      match fs::create_dir(&self.directory) {
        Ok(_) => (),
        Err(error) => return Err(error),
      }
    }

    Ok(())
  }

  fn ensure_repository_git(&self) -> Result<(), git2::Error> {
    // check if directory is already a git repository
    match git::init(
      &self.directory,
      &self.config.git_options.url.clone().unwrap(),
    ) {
      Ok(()) => (),
      Err(error) => {
        return Err(error);
      },
    };

    // update repository
    match git::update(&self.directory, &self.config.git_options) {
      Ok(()) => (),
      Err(error) => {
        return Err(error);
      }
    }

    Ok(())
  }

  fn load_templates(&mut self) {
    let mut templates = Vec::<template::Template>::new();

    // check if folder exists
    match fs::read_dir(&self.directory) {
      Ok(fc) => fc,
      Err(_error) => return,
    };

    // Loop at all entries in templates directory
    for entry in fs::read_dir(&self.directory).unwrap() {
      let entry = &entry.unwrap();
      // check if entry is file, if yes skip entry
      if !entry.path().is_dir() {
        continue;
      }

      let meta = match meta::load_meta(&entry.path()) {
        Ok(meta) => meta,
        Err(error) => {
          log::error!("{}", error);
          continue;
        }
      };

      // If type is None or unqual template skip entry
      if meta.kind != String::from("template") {
        continue;
      }

      let template = match template::Template::new(&entry) {
        Ok(template) => template,
        Err(_error) => continue,
      };

      templates.push(template);
    }
    self.templates = templates;
  }
}
