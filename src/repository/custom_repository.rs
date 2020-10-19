use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::io::{Error, Write};
use std::path::{Path, PathBuf};

use crate::config::{Config, RepositoryOptions};
use crate::context::Context;
use crate::error::RunError;
use crate::git;
use crate::meta;
use crate::repository::{Repository, CopyOptions};
use crate::template;
use crate::template::{renderer, Template};
use crate::utils;

#[derive(Debug)]
pub struct CustomRepository {
  pub config: RepositoryOptions,
  pub directory: PathBuf,
  pub meta: Option<meta::Meta>,
  pub templates: Vec<template::Template>,
}

impl Repository for CustomRepository {
  fn get_config(&self) -> RepositoryOptions {
    return self.config.clone();
  }

  fn copy_template(&self, ctx: &Context, opts: &CopyOptions) -> Result<(), RunError> {
    let template = self.get_template_by_name(&opts.template_name)?;

    let super_templates =
      self.get_super_templates(template, &mut std::collections::HashSet::new())?;

    // Initialize super templates
    for template in super_templates.iter() {
      template.init(ctx, &opts.target, &opts.render_context)?;
    }

    // Initialize template
    template.init(ctx, &opts.target, &opts.render_context)?;

    // Create info file
    template.create_info(&opts.target)?;

    Ok(())
  }

  fn get_template_values(&self, template_name: &str) -> Result<HashSet<String>, RunError> {
    let template = self.get_template_by_name(&template_name)?;

    // Get list of all super templates
    let super_templates = match self.get_super_templates(template, &mut HashSet::new()) {
      Ok(templates) => templates,
      Err(error) => return Err(error),
    };

    let mut values = HashSet::new();
    for template in super_templates {
      values.extend(template.meta.get_values());
    }

    values.extend(template.meta.get_values());

    Ok(values)
  }

  /// Return list of all template names in this repository
  fn get_template_names(&self) -> Vec<String> {
    let mut templates = Vec::<String>::new();

    for template in &self.templates {
      templates.push(utils::lowercase(&template.name));
    }

    return templates;
  }

  /// Return template with given name
  fn get_template_by_name(&self, name: &str) -> Result<&template::Template, RunError> {
    for template in &self.templates {
      if template.name == *name {
        return Ok(template);
      }
    }

    return Err(RunError::Template(String::from("Not found")));
  }
}

impl CustomRepository {
  pub fn new(config: &Config, name: &str) -> Result<CustomRepository, RunError> {
    log::info!("Loading repository: {}", name);
    let cfg = match config.get_repository_config(name) {
      Option::Some(cfg) => cfg,
      Option::None => {
        return Err(RunError::Repository(String::from("Not found")));
      }
    };

    let directory = config.repositories_dir.join(&utils::lowercase(name));

    let mut repository = CustomRepository {
      config: cfg,
      directory: directory,
      meta: None,
      templates: Vec::<template::Template>::new(),
    };

    // Ensure repository diectory
    match repository.ensure_repository_dir() {
      Ok(()) => (),
      Err(error) => {
        log::error!("{}", error);
        return Err(RunError::Repository(String::from("Initialization")));
      }
    };

    // Ensure git setup if enabled
    if repository.config.git_options.enabled {
      match repository.ensure_repository_git() {
        Ok(()) => (),
        Err(_) => (),
      };
    }

    // Load meta
    repository.load_meta()?;

    // Load templates
    repository.load_templates()?;

    return Ok(repository);
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

  /// Get list of all super templates
  fn get_super_templates(
    &self,
    template: &template::Template,
    seen: &mut std::collections::HashSet<String>,
  ) -> Result<Vec<Template>, RunError> {
    // get list of all super templates
    let super_templates = template.get_super_templates()?;

    seen.insert(template.name.to_owned());

    let mut templates = vec![];
    for name in super_templates {
      // Avoid circular dependencies;
      if seen.contains(&name) {
        continue;
      }

      let template = self.get_template_by_name(&name)?;

      let t = match self.get_super_templates(template, seen) {
        Ok(templates) => templates,
        Err(error) => return Err(error),
      };

      templates.extend(t);
      templates.push(template.to_owned());
    }

    Ok(templates)
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
    // initialize git repository
    match git::init(
      &self.directory,
      &self.config.git_options.url.clone().unwrap(),
    ) {
      Ok(()) => (),
      Err(error) => {
        return Err(error);
      }
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

  fn load_meta(&mut self) -> Result<(), RunError> {
    self.meta = match meta::load(&self.directory) {
      Ok(meta) => Some(meta),
      Err(error) => {
        log::error!("{}", error);
        eprintln!("{}", error);
        return Err(RunError::Repository(String::from("Unable to load meta")));
      }
    };

    Ok(())
  }

  fn load_templates(&mut self) -> Result<(), RunError> {
    self.templates = Vec::<template::Template>::new();

    // check if folder exists
    match fs::read_dir(&self.directory) {
      Ok(fc) => fc,
      Err(error) => return Err(RunError::IO(error)),
    };

    // Loop at all entries in repository directory
    for entry in fs::read_dir(&self.directory).unwrap() {
      let entry = &entry.unwrap();
      // check if entry is file, if yes skip entry
      if !entry.path().is_dir() {
        continue;
      }

      let meta = match meta::load(&entry.path()) {
        Ok(meta) => meta,
        Err(error) => {
          log::error!("{}", error);
          continue;
        }
      };

      // Skip if type is not template
      if meta.kind != meta::Type::TEMPLATE {
        continue;
      }

      let template = match template::Template::new(&entry.path()) {
        Ok(template) => template,
        Err(error) => {
          log::error!("{}", error);
          continue;
        }
      };

      self.templates.push(template);
    }

    Ok(())
  }
}

pub fn add(config: &Config, options: &RepositoryOptions) -> Result<(), RunError> {
  let directory = Path::new(&config.repositories_dir).join(&utils::lowercase(&options.name));

  let repository = CustomRepository {
    config: options.clone(),
    directory: directory,
    meta: None,
    templates: vec![],
  };

  // Ensure repository diectory
  match repository.ensure_repository_dir() {
    Ok(()) => (),
    Err(error) => {
      log::error!("{}", error);
      return Err(RunError::Repository(String::from("Initialization")));
    }
  };

  // Ensure git setup if enabled
  if repository.config.git_options.enabled {
    match repository.ensure_repository_git() {
      Ok(()) => (),
      Err(error) => {
        log::error!("{}", error);
        return Err(RunError::Git(String::from("Initialization")));
      }
    };
  };

  Ok(())
}

pub fn create(directory: &Path, options: &RepositoryOptions) -> Result<(), RunError> {
  let directory = directory.join(&utils::lowercase(&options.name));

  // Create repository directory
  fs::create_dir(&directory)?;

  // Create meta data
  let mut meta = meta::Meta::new(meta::Type::REPOSITORY);
  meta.name = options.name.to_owned();
  meta.description = options.description.to_owned();

  // Create meta.json
  let meta_path = directory.join("meta.json");
  let mut meta_file = File::create(meta_path)?;

  // Create meta data
  let meta_data = serde_json::to_string_pretty(&meta).unwrap();
  match meta_file.write(meta_data.as_bytes()) {
    Ok(_) => (),
    Err(error) => return Err(RunError::IO(error)),
  };

  // Initialize git repository
  if options.git_options.enabled && options.git_options.url.is_some() {
    match git::init(&directory, &options.git_options.url.clone().unwrap()) {
      Ok(()) => (),
      Err(error) => {
        log::error!("{}", error);
        return Err(RunError::Git(String::from("Initialization")));
      }
    };
  }

  Ok(())
}
