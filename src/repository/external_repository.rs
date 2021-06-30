use linked_hash_set::LinkedHashSet;
use log;
use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::io::{Write};
use std::path::{Path, PathBuf};

use crate::config::{Config, RepositoryOptions};
use crate::context::Context;
use crate::error::RunError;
use crate::meta::{self, RepositoryMeta, TemplateMeta, Value};
use crate::repository::{CopyOptions, Repository};
use crate::template::Template;
use crate::utils;

#[derive(Debug)]
pub struct ExternalRepository {
  pub config: RepositoryOptions,
  pub directory: PathBuf,
  pub meta: Option<RepositoryMeta>,
  pub templates: Vec<Template>,
}

impl Repository for ExternalRepository {
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

  fn get_template_by_name(&self, name: &str) -> Result<&Template, RunError> {
    for template in &self.templates {
      if template.name == *name {
        return Ok(template);
      }
    }

    return Err(RunError::Template(String::from("Not found")));
  }
}

impl ExternalRepository {
  pub fn new(config: &Config, name: &str) -> Result<ExternalRepository, RunError> {
    log::info!("Loading repository: {}", name);
    let cfg = match config.get_repository_config(name) {
      Option::Some(cfg) => cfg,
      Option::None => {
        return Err(RunError::Repository(String::from("Not found")));
      }
    };

    let directory = match cfg.directory.clone() {
      Some(directory) => directory,
      None => return Err(RunError::Config(String::from("Repository directory empty")))
    };

    let mut repository = ExternalRepository {
      config: cfg,
      directory: Path::new(&directory).to_owned(),
      meta: None,
      templates: Vec::<Template>::new(),
    };

    // Ensure repository diectory
    if !repository.directory.exists() {
      log::error!("{}", "Directory does not exists");
      return Err(RunError::Repository(String::from("Initialization")));
    }

    // Load meta
    repository.load_meta()?;

    // Load templates
    repository.load_templates()?;

    return Ok(repository);
  }

  /// Get list of all super templates
  fn get_super_templates(
    &self,
    template: &Template,
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

  fn load_templates(&mut self) -> Result<(), RunError> {
    self.templates = Vec::<Template>::new();

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

      let template = match Template::new(&entry.path()) {
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

  Ok(())
}
