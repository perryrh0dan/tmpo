use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::io::{ErrorKind, Read, Write};
use log;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use crate::error::RunError;
use crate::repository::Repository;
use crate::utils;
use crate::meta;

extern crate serde;
use serde::{Serialize};

pub mod context;
mod renderer;

#[derive(Serialize, Debug)]
pub struct Info {
  name: String,
  version: Option<String>
}

#[derive(Debug, Clone)]
pub struct Template {
  pub name: String,
  pub path: PathBuf,
  pub meta: meta::Meta,
}

impl Template {
  pub fn new(dir: &std::fs::DirEntry) -> Result<Template, RunError> {
    let meta = match meta::load_meta(&dir.path()) {
      Ok(meta) => meta,
      Err(error) => {
        log::error!("{}", error);
        eprintln!("{}", error);
        return Err(RunError::Template(String::from("Unable to load meta")));
      }
    };

    // If type is None or unqual template skip entry
    if meta.kind.is_none() || meta.kind != Some(String::from("template")) {
      return Err(RunError::Template(String::from("Initialization")));
    }

    let name;
    if meta.name.is_none() {
      name = dir
        .path()
        .file_name()
        .unwrap()
        .to_string_lossy()
        .into_owned();
    } else {
      name = meta.name.clone().unwrap();
    }

    // make all names lowercase
    return Ok(Template{
      name: utils::lowercase(&name),
      path: dir.path(),
      meta: meta,
    });
  }

  pub fn copy(&self, repository: &Repository, target: &Path, opts: &context::Context) -> Result<(), RunError> {
    // Get list of all super templates
    let super_templates = match self.get_super_templates(repository) {
      Ok(templates) => templates,
      Err(error) => return Err(error),
    };

    for template in super_templates {
      template.copy(repository, target, opts)?;
    }

    // Run before install script
    if self.meta.scripts.is_some() && self.meta.scripts.as_ref().unwrap().before_install.is_some() {
      let script = self.meta.scripts
        .as_ref()
        .unwrap()
        .before_install
        .as_ref()
        .unwrap();
      let script = renderer::render(script, &opts);

      run_script(&script, target);
    }

    // Copy files
    self.copy_folder(&self.path, &target, &opts)?;

    // Create meta file
    self.create_meta(&target)?;

    // Run after install script
    if self.meta.scripts.is_some() && self.meta.scripts.as_ref().unwrap().after_install.is_some() {
      let script = self.meta.scripts
        .as_ref()
        .unwrap()
        .after_install
        .as_ref()
        .unwrap();
      let script = renderer::render(script, &opts);

      run_script(&script, target);
    }

    Ok(())
  }

  fn copy_folder(&self, src: &Path, target: &Path, opts: &context::Context) -> Result<(), RunError> {
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

  pub fn get_custom_values(&self, repository: &Repository) -> Result<HashSet<String>, RunError> {
    // Get list of all super templates
    let super_templates = match self.get_super_templates(repository) {
      Ok(templates) => templates,
      Err(error) => return Err(error),
    };

    let mut values = HashSet::new();
    for template in super_templates {
      values.extend(template.meta.get_values());
    };

    values.extend(self.meta.get_values());

    Ok(values)
  }

  /// Get list of all super templates
  fn get_super_templates(&self, repository: &Repository) -> Result<Vec<Template>, RunError> {
    // get list of all super templates
    let super_templates = match &self.meta.extend {
      None => Vec::new(),
      Some(x) => x.clone(),
    };

    let mut templates = vec!{};
    for name in super_templates {
      let template = match repository.get_template_by_name(&name) {
        Ok(template) =>  template,
        Err(error) => {
          log::error!("{}", error);
          return Err(RunError::Template(String::from("Not found")));
        },
      };

      let t = match template.get_super_templates(repository) {
        Ok(templates) => templates,
        Err(error) => return Err(error),
      };

      templates.extend(t);
      templates.push(template.to_owned());
    }

    Ok(templates)
  }

  fn is_excluded_renderer(&self, name: &str) -> bool {
    if self.meta.renderer.is_none() {
      return false;
    }

    let items = match &self.meta.renderer.as_ref().unwrap().exclude {
      None => return false,
      Some(x) => x,
    };

    self.is_excluded(name, items)
  }

  fn is_excluded_copy(&self, name: &str) -> bool {
    if name == "meta.json" {
      return true;
    };

    let items = match &self.meta.exclude {
      None => return false,
      Some(x) => x,
    };

    self.is_excluded(name, items)
  }

  fn is_excluded(&self, name: &str, items: &Vec<String>) -> bool {
    // Check if excluded
    for item in items.iter() {
      if item == &name {
        return true;
      }
    }

    return false;
  }

  fn create_meta(&self, target: &Path) -> Result<(), std::io::Error> {
    // Create .tmpo.yaml file
    // Not used yes
    let meta_path = &target.to_path_buf().join(".tmpo.yaml");
    let mut meta_file = fs::File::create(meta_path)?;

    // Fill meta
    let info = Info {
      name: self.name.to_owned(),
      version: self.meta.version.clone(),
    };

    let meta_data = serde_yaml::to_string(&info).unwrap();
    meta_file.write(meta_data.as_bytes())?;

    Ok(())
  }
}

fn run_script(script: &String, target: &Path) {
  // Check if script is empty
  if script == "" {
    return;
  }

  log::info!("Run script: {}", script);

  let mut cmd = if cfg!(target_os = "windows") {
    Command::new("cmd")
      .current_dir(target)
      .arg(script)
      .stdout(Stdio::inherit())
      .stderr(Stdio::inherit())
      .spawn()
      .expect("failed to execute process")
  } else {
    Command::new("sh")
      .current_dir(target)
      .arg("-c")
      .arg(script)
      .stdout(Stdio::inherit())
      .stderr(Stdio::inherit())
      .spawn()
      .expect("failed to execute process")
  };

  let status = match cmd.wait() {
    Ok(status) => status,
    Err(error) => {
      log::error!("Script exited with error: {}", error);
      return;
    },
  };

  log::info!("Script exit status: {}", status);
}


