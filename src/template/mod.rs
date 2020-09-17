use log;
use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::io::{ErrorKind, Read, Write};
use std::path::{Path, PathBuf};

use crate::error::RunError;
use crate::meta;
use crate::repository::Repository;
use crate::utils;

extern crate serde;
use serde::Serialize;

pub mod renderer;
mod script;

#[derive(Serialize, Debug)]
pub struct Info {
  name: String,
  version: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Template {
  pub name: String,
  pub path: PathBuf,
  pub meta: meta::Meta,
}

impl Template {
  pub fn new(dir: &Path) -> Result<Template, RunError> {
    let meta = match meta::load(&dir) {
      Ok(meta) => meta,
      Err(error) => {
        log::error!("{}", error);
        eprintln!("{}", error);
        return Err(RunError::Template(String::from("Unable to load meta")));
      }
    };

    // Check if type is Template
    if meta.kind != meta::Type::TEMPLATE {
      return Err(RunError::Template(String::from("Initialization")));
    }

    let name = meta.name.to_owned();

    // make all names lowercase
    return Ok(Template {
      name: utils::lowercase(&name),
      path: dir.to_path_buf(),
      meta: meta,
    });
  }

  /// Create a new template with given name in the repository directory
  pub fn create(dir: &Path, meta: &meta::Meta)-> Result<std::path::PathBuf, RunError> {
    let template_path = dir.join(utils::lowercase(&meta.name));

    // Create template directory
    fs::create_dir(&template_path)?;

    // Create meta.json
    let meta_path = template_path.join("meta.json");
    let mut meta_file = File::create(meta_path)?;

    let meta_data = serde_json::to_string_pretty(&meta).unwrap();
    meta_file.write(meta_data.as_bytes())?;

    return Ok(template_path);
  }

  pub fn copy(
    &self,
    repository: &Repository,
    target: &Path,
    opts: &renderer::Context,
  ) -> Result<(), RunError> {
    // Get list of all super templates
    let super_templates = match self.get_super_templates(repository, &mut HashSet::new()) {
      Ok(templates) => templates,
      Err(error) => return Err(error),
    };

    // Initialize super templates
    for template in super_templates {
      template.init(target, opts)?;
    }

    // Initialize template
    self.init(target, opts)?;

    // Create info file
    self.create_info(&target)?;

    Ok(())
  }

  pub fn get_custom_values(&self, repository: &Repository) -> Result<HashSet<String>, RunError> {
    // Get list of all super templates
    let super_templates = match self.get_super_templates(repository, &mut HashSet::new()) {
      Ok(templates) => templates,
      Err(error) => return Err(error),
    };

    let mut values = HashSet::new();
    for template in super_templates {
      values.extend(template.meta.get_values());
    }

    values.extend(self.meta.get_values());

    Ok(values)
  }

  fn init(&self, target: &Path, opts: &renderer::Context) -> Result<(), RunError> {
    log::info!("Initialize Template: {}", self.name);

    // Run before install script
    let before_install_script = self.meta.get_before_install_script();
    if before_install_script.is_some() {
      let script = renderer::render(&before_install_script.unwrap(), &opts);

      script::run(&script, target);
    }

    // Copy files
    self.copy_folder(&self.path, &target, &opts)?;

    // Run after install script
    let after_install_script = self.meta.get_after_install_script();
    if after_install_script.is_some() {
      let script = renderer::render(&after_install_script.unwrap(), &opts);

      script::run(&script, target);
    }

    Ok(())
  }

  fn copy_folder(
    &self,
    src: &Path,
    target: &Path,
    opts: &renderer::Context,
  ) -> Result<(), RunError> {
    // Loop at selected template directory
    let entries = match fs::read_dir(src) {
      Ok(values) => values,
      Err(error) => return Err(RunError::IO(error)),
    };

    for entry in entries {
      let entry = &entry.unwrap();

      let source_path = &entry.path();
      let source_name = &entry
        .path()
        .file_name()
        .unwrap()
        .to_string_lossy()
        .into_owned();

      let mut path = target.to_path_buf();
      path.push(source_name);

      // Replace placeholders in path
      path = PathBuf::from(renderer::render(&path.to_string_lossy(), &opts));

      // Check if entry is directory
      if entry.path().is_dir() {
        match fs::create_dir(&path) {
          Ok(()) => (),
          Err(error) => match error.kind() {
            ErrorKind::AlreadyExists => (),
            _ => return Err(RunError::IO(error)),
          },
        };

        self.copy_folder(&source_path, &path, opts)?
      } else {
        if self.is_excluded_copy(&source_name) {
          continue;
        }

        // Open file
        let mut src = File::open(Path::new(&source_path))?;
        let mut data = String::new();

        // Write to data string
        src.read_to_string(&mut data)?;

        // Close file
        drop(src);

        // Replace placeholders in data
        // Skip if file should be excluded
        if !self.is_excluded_renderer(&source_name) {
          data = renderer::render(&data, &opts);
        }

        // Create file
        let mut dst = File::create(path)?;
        dst.write(data.as_bytes())?;

        // Close file
        drop(dst);
      }
    }

    Ok(())
  }

  /// Get list of all super templates
  pub fn get_super_templates(
    &self,
    repository: &Repository,
    seen: &mut std::collections::HashSet<String>,
  ) -> Result<Vec<Template>, RunError> {
    // get list of all super templates
    let super_templates = match &self.meta.extend {
      None => Vec::new(),
      Some(x) => x.clone(),
    };

    seen.insert(self.meta.name.to_owned());

    let mut templates = vec![];
    for name in super_templates {
      // Avoid circular dependencies;
      if seen.contains(&name) {
        continue;
      }

      let template = match repository.get_template_by_name(&name) {
        Ok(template) => template,
        Err(error) => {
          log::error!("{}", error);
          return Err(RunError::Template(String::from("Not found")));
        }
      };

      let t = match template.get_super_templates(repository, seen) {
        Ok(templates) => templates,
        Err(error) => return Err(error),
      };

      templates.extend(t);
      templates.push(template.to_owned());
    }

    Ok(templates)
  }

  fn is_excluded_copy(&self, name: &str) -> bool {
    if name == "meta.json" {
      return true;
    };

    let items = match &self.meta.exclude {
      None => return false,
      Some(x) => x,
    };

    items.contains(&name.to_owned())
  }

  fn is_excluded_renderer(&self, name: &str) -> bool {
    if self.meta.renderer.is_none() {
      return false;
    }

    let items = match &self.meta.renderer.as_ref().unwrap().exclude {
      None => return false,
      Some(x) => x,
    };

    items.contains(&name.to_owned())
  }

  fn create_info(&self, target: &Path) -> Result<(), std::io::Error> {
    // Create .tmpo.yaml file
    // Not used yet
    let info_path = &target.to_path_buf().join(".tmpo.yaml");
    let mut info_file = fs::File::create(info_path)?;

    // Fill meta
    let info = Info {
      name: self.name.to_owned(),
      version: self.meta.version.clone(),
    };

    let info_data = serde_yaml::to_string(&info).unwrap();
    info_file.write(info_data.as_bytes())?;

    Ok(())
  }
}
