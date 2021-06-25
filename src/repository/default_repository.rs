use linked_hash_set::LinkedHashSet;
use std::fs;
use log;
use std::io::{Error};
use std::path::{PathBuf};

use crate::config::{Config, RepositoryOptions, TemplateOptions};
use crate::context::Context;
use crate::error::RunError;
use crate::git;
use crate::meta::{self, TemplateMeta, Value};
use crate::repository::{CopyOptions, Repository};
use crate::template;
use crate::utils;

#[derive(Debug)]
pub struct DefaultRepository {
  pub directory: PathBuf,
  pub templates: Vec<template::Template>,
}

impl Repository for DefaultRepository {
  fn get_config(&self) -> RepositoryOptions {
    return RepositoryOptions {
      name: String::from("Templates"),
      kind: Some(String::from("default")),
      directory: None,
      description: Some(String::from("Mono repository templates")),
      git_options: git::Options::new(),
    };
  }

  fn copy_template(&self, ctx: &Context, opts: &CopyOptions) -> Result<(), RunError> {
    let template = self.get_template_by_name(&opts.template_name)?;

    // Initialize template
    template.init(ctx, &opts.target, &opts.render_context)?;

    // Create info file
    template.create_info(&opts.target)?;

    Ok(())
  }

  fn get_template_info(&self, template_name: &str) -> Result<Option<String>, RunError> {
    let template = self.get_template_by_name(template_name)?;

    Ok(template.meta.info.to_owned())
  }

  fn get_template_values(&self, template_name: &str) -> Result<LinkedHashSet<Value>, RunError> {
    let template = self.get_template_by_name(template_name)?;

    let mut values = LinkedHashSet::new();
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

impl DefaultRepository {
  pub fn new(config: &Config) -> Result<DefaultRepository, RunError> {
    log::info!("Loading repository: Templates");

    let directory = PathBuf::from(&config.templates_dir);

    let mut repository = DefaultRepository {
      directory: directory,
      templates: Vec::<template::Template>::new(),
    };

    for template in &config.templates {
      match repository.ensure_template_dir(&template.name) {
        Ok(()) => (),
        Err(error) => {
          log::error!("{}", error);
        }
      };

      match repository.ensure_template_git(&template) {
        Ok(()) => (),
        Err(error) => {
          log::error!("{}", error);
        }
      };
    }

    // Load templates
    repository.load_templates()?;

    return Ok(repository);
  }

  pub fn remove_template(&self, template_name: &str) -> Result<(), RunError> {
    let template = self.get_template_by_name(template_name)?;

    log::info!(
      "Delete template directory {}",
      template.path.to_owned().to_str().unwrap()
    );

    match fs::remove_dir_all(&template.path) {
      Ok(()) => (),
      Err(error) => {
        return Err(RunError::IO(error));
      }
    }

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

  fn ensure_template_dir(&self, template: &str) -> Result<(), Error> {
    let path = self.directory.join(&template);

    if !path.exists() {
      match fs::create_dir(&self.directory) {
        Ok(_) => (),
        Err(error) => return Err(error),
      }
    }

    Ok(())
  }

  fn ensure_template_git(&self, template: &TemplateOptions) -> Result<(), git2::Error> {
    let path = self.directory.join(&template.name);

    if template.git_options.url.is_none() {
      return Ok(())
    }

    // initialize git repository
    match git::init(
      &path,
      &template.git_options.url.clone().unwrap(),
    ) {
      Ok(()) => (),
      Err(error) => {
        return Err(error);
      }
    };

    // update repository
    match git::update(&path, &template.git_options) {
      Ok(()) => (),
      Err(error) => {
        return Err(error);
      }
    }

    Ok(())
  }
}

pub fn add(config: &Config, options: TemplateOptions) -> Result<Config, RunError> {
  let mut new_config = config.clone();
  new_config.templates.push(options);

  Ok(new_config)
}
