use std::fs;
use std::io::Error;
use std::path::Path;

use crate::config::Config;
use crate::git;
use crate::renderer;

extern crate custom_error;
use custom_error::custom_error;

pub mod template;

pub struct Repository {
  pub directory: String,
  pub templates: Vec<template::Template>,
}

custom_error! {pub RepositoryError
  InitializationError = "Unable to initialize repository",
  TemplateNotFound = "Unable to find template",
  LoadingErrors = "Unable to load templates",
}

impl Repository {
  pub fn new(config: &Config, verbose: bool) -> Result<Repository, RepositoryError> {
    match ensure_template_dir(config, verbose) {
      Ok(()) => (),
      Err(_error) => return Err(RepositoryError::InitializationError),
    };

    let templates = match load_templates(&config.templates_dir) {
      Ok(templates) => templates,
      Err(_error) => return Err(RepositoryError::LoadingErrors),
    };

    return Ok(Repository {
      directory: config.templates_dir.clone(),
      templates: templates,
    })
  }

  pub fn get_template_by_name(&self, name: &String) -> Result<&template::Template, RepositoryError> {
    for template in &self.templates {
      if template.name == *name {
        return Ok(template);
      }
    }

    return Err(RepositoryError::TemplateNotFound);
  }
}

fn ensure_template_dir(config: &Config, verbose: bool) -> Result<(), Error> {
  // Create if dir not exists
  let r = fs::create_dir_all(Path::new(&config.templates_dir));
  match r {
    Ok(fc) => fc,
    Err(error) => return Err(error),
  }

  // Initialize git repository if enabled
  if config.templates_repo.enabled {
    match git::init(&config.templates_dir, &config.templates_repo.url) {
      Ok(()) => (),
      Err(error) => match error {
        git::GitError::InitError => println!("Init Error"),
        git::GitError::AddRemoteError => println!("Add Remote Error"),
      },
    };

    match git::update(&config.templates_dir, &config.templates_repo, verbose) {
      Ok(()) => (),
      Err(_e) => renderer::errors::update_templates(),
    }
  }

  Ok(())
}

fn load_templates(directory: &String) -> Result<Vec<template::Template>, Error> {
  let mut templates = Vec::<template::Template>::new();

  // check if folder exists
  match fs::read_dir(directory) {
    Ok(fc) => fc,
    Err(error) => return Err(error),
  };

  // Loop at all entries in templates directory
  for entry in fs::read_dir(directory).unwrap() {
    let entry = &entry.unwrap();
    // check if entry is file, if yes skip entry
    if !entry.path().is_dir() {
      continue;
    }

    let entry_path = entry.path().to_string_lossy().into_owned();
    let entry_meta = template::meta::load_meta(&entry_path)?;

    // If type is None or unqual template skip entry
    if entry_meta.kind.is_none() || entry_meta.kind != Some(String::from("template")) {
      continue;
    }

    let template = match template::Template::new(&entry) {
      Ok(template) => template,
      Err(_error) => continue,
    };

    templates.push(template);
  }

  return Ok(templates);
}
