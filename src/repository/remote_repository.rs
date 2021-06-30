use linked_hash_set::LinkedHashSet;
use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::io::{Error, Write};
use std::path::{Path, PathBuf};

use crate::config::{Config, RepositoryOptions};
use crate::context::Context;
use crate::error::RunError;
use crate::git;
use crate::meta::{self, RepositoryMeta, TemplateMeta, Value};
use crate::repository::{CopyOptions, Repository};
use crate::template;
use crate::template::Template;
use crate::utils;

#[derive(Debug)]
pub struct RemoteRepository {
  pub config: RepositoryOptions,
  pub directory: PathBuf,
  pub meta: Option<RepositoryMeta>,
  pub templates: Vec<template::Template>,
}

impl Repository for RemoteRepository {
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

  fn get_template_info(&self, template_name: &str) -> Result<Option<String>, RunError> {
    let template = self.get_template_by_name(template_name)?;

    if template.meta.info.is_some() {
      return Ok(template.meta.info.to_owned());
    }

    // Get list of all super templates
    let super_templates = match self.get_super_templates(template, &mut HashSet::new()) {
      Ok(templates) => templates,
      Err(error) => return Err(error),
    };

    for template in super_templates.iter().rev() {
      if template.meta.info.is_some() {
        return Ok(template.meta.info.to_owned());
      }
    }

    Ok(None)
  }

  fn get_template_values(&self, template_name: &str) -> Result<LinkedHashSet<Value>, RunError> {
    let template = self.get_template_by_name(&template_name)?;

    // Get list of all super templates
    let super_templates = match self.get_super_templates(template, &mut HashSet::new()) {
      Ok(templates) => templates,
      Err(error) => return Err(error),
    };

    let mut values = LinkedHashSet::new();
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
      if template.meta.visible.is_some() && template.meta.visible.unwrap() == false {
        continue;
      }

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

impl RemoteRepository {
  pub fn new(config: &Config, name: &str) -> Result<RemoteRepository, RunError> {
    log::info!("Loading repository: {}", name);
    let cfg = match config.get_repository_config(name) {
      Option::Some(cfg) => cfg,
      Option::None => {
        return Err(RunError::Repository(String::from("Not found")));
      }
    };

    let directory = config.repositories_dir.join(&utils::lowercase(name));

    let mut repository = RemoteRepository {
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
    if repository.config.git_options.clone().unwrap().enabled {
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
    let git_options = &self.config.git_options.clone().unwrap();

    let valid = git::check(
      &self.directory,
      &git_options.url.clone().unwrap(),
    );

    if !valid {
      match git::init(
        &self.directory,
        &git_options.url.clone().unwrap(),
      ) {
        Ok(()) => (),
        Err(error) => {
          return Err(error);
        }
      };
    }

    // update repository
    match git::update(&self.directory, &git_options) {
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

      let meta = match meta::load::<TemplateMeta>(&entry.path()) {
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

  let repository = RemoteRepository {
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
  if repository.config.git_options.clone().unwrap().enabled {
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
  let mut meta = meta::RepositoryMeta::new(meta::Type::REPOSITORY);
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
  let git_options = options.git_options.clone().unwrap();
  if git_options.enabled && git_options.url.is_some() {
    match git::init(&directory, &git_options.url.unwrap()) {
      Ok(()) => (),
      Err(error) => {
        log::error!("{}", error);
        return Err(RunError::Git(String::from("Initialization")));
      }
    };
  }

  Ok(())
}
