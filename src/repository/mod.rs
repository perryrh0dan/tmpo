use std::fs;
use std::io::{Error};
use std::path::{PathBuf};

use crate::config::{Config, RepositoryOptions};
use crate::git;
use crate::out;
use crate::template;
use crate::meta;
use crate::utils;

extern crate custom_error;
use custom_error::custom_error;

pub struct Repository {
  pub directory: String,
  pub config: RepositoryOptions,
  pub templates: Vec<template::Template>,
}

custom_error! {pub RepositoryError
  InitializationError = "Unable to initialize repository",
  NotFound = "Repository not found",
  TemplateNotFound = "Unable to find template",
  LoadingErrors = "Unable to load templates",
}

impl Repository {
  pub fn new(config: &Config, name: &str) -> Result<Repository, RepositoryError> {
    let cfg = match config.get_repository_config(name) {
      Option::Some(cfg) => cfg,
      Option::None => return Err(RepositoryError::NotFound),
    };

    let directory = String::from(&config.templates_dir) + "/" + &name;

    let mut repository = Repository {
      directory: directory,
      config: cfg,
      templates: Vec::<template::Template>::new(),
    };

    match repository.ensure_repository_dir(&config) {
      Ok(()) => (),
      Err(_error) => return Err(RepositoryError::InitializationError),
    };

    repository.load_templates();

    return Ok(repository);
  }

  pub fn get_repositories(config: &Config) -> Vec<String> {
    let mut repositories = Vec::<String>::new();

    for entry in &config.templates_repositories {
      repositories.push(String::from(&entry.name));
    }

    return repositories;
  }

  pub fn delete_repository(config: &Config, name: &str) -> Result<(), Error> {
    let mut repository_dir = PathBuf::from(&config.templates_dir);
    repository_dir.push(&name);

    match fs::remove_dir_all(repository_dir) {
      Ok(()) => (),
      Err(error) => return Err(error)
    }

    return Ok(());
  }

  pub fn get_templates(&self) -> Vec<String> {
    let mut templates = Vec::<String>::new();

    for template in &self.templates {
      templates.push(String::from(&template.name));
    }

    return templates;
  }

  pub fn get_template_by_name(
    &self,
    name: &str,
  ) -> Result<&template::Template, RepositoryError> {
    for template in &self.templates {
      if template.name == *name {
        return Ok(template);
      }
    }

    return Err(RepositoryError::TemplateNotFound);
  }

  fn ensure_repository_dir(&self, config: &Config) -> Result<(), Error> {
    let mut directory = PathBuf::from(&config.templates_dir);
    directory.push(&utils::lowercase(&self.config.name));
    let r = fs::create_dir_all(&directory);
    match r {
      Ok(fc) => fc,
      Err(error) => return Err(error),
    }

    // Initialize git repository if enabled
    if self.config.git_options.enabled {
      match git::init(&directory, &self.config.git_options.url.clone().unwrap()) {
        Ok(()) => (),
        Err(error) => match error {
          git::GitError::InitError => println!("Init Error"),
          git::GitError::AddRemoteError => println!("Add Remote Error"),
        },
      };
      match git::update(&directory, &self.config.git_options) {
        Ok(()) => (),
        Err(_e) => out::errors::update_templates(),
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

      let path = entry.path().to_string_lossy().into_owned();
      let meta = meta::load_meta(&path).unwrap();

      // If type is None or unqual template skip entry
      if meta.kind.is_none() || meta.kind != Some(String::from("template")) {
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
