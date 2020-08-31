use std::fs;
use std::fs::File;
use std::io::{Error, ErrorKind, Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use crate::repository::Repository;
use crate::utils;
use crate::meta;

extern crate custom_error;
use custom_error::custom_error;
extern crate serde;
use serde::{Serialize};

pub mod context;
mod renderer;

custom_error! {pub TemplateError
  InitializeTemplate = "Unable to initialize template"
}

#[derive(Serialize)]
pub struct Info {
  name: String,
  version: Option<String>
}

pub struct Template {
  pub name: String,
  pub path: String,
  pub meta: meta::Meta,
}

impl Template {
  pub fn new(dir: &std::fs::DirEntry) -> Result<Template, TemplateError> {
    let path = dir.path().to_string_lossy().into_owned();
    let meta = meta::load_meta(&path).unwrap();

    // If type is None or unqual template skip entry
    if meta.kind.is_none() || meta.kind != Some(String::from("template")) {
      return Err(TemplateError::InitializeTemplate);
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
    return Ok(Template {
      name: utils::lowercase(&name),
      path: path,
      meta: meta,
    });
  }

  pub fn copy(&self, repository: &Repository, target: &Path, opts: context::Context) -> Result<(), Error> {
    // get list of all super templates
    let super_templates = match &self.meta.extend {
      None => Vec::new(),
      Some(x) => x.clone(),
    };

    for name in super_templates {
      let template = repository.get_template_by_name(&name).unwrap();
      let opts = opts.clone();
      template.copy(repository, target, opts)?;
    }

    // run before install script
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

    // copy files
    self.copy_folder(&self.path, &target, &opts)?;

    // create meta file
    self.create_meta(&target)?;

    // run after install script
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

  fn copy_folder(&self, src: &str, target: &Path, opts: &context::Context) -> Result<(), Error> {
    // Loop at selected template directory
    for entry in fs::read_dir(src)? {
      let entry = &entry.unwrap();

      let source_path = &entry.path().to_string_lossy().into_owned();
      let source_name = &entry
        .path()
        .file_name()
        .unwrap()
        .to_string_lossy()
        .into_owned();

      let mut path = target.to_path_buf();
      path.push(source_name);

      // replace placeholders in path
      path = PathBuf::from(renderer::render(&path.to_string_lossy(), &opts));

      // check if entry is directory
      if entry.path().is_dir() {
        match fs::create_dir(&path) {
          Ok(()) => (),
          Err(error) => match error.kind() {
            ErrorKind::AlreadyExists => (),
            _ => return Err(error),
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

        // close file
        drop(src);

        // replace placeholders in data
        // skip if file should be excluded
        if !self.is_excluded_renderer(&source_name) {
          data = renderer::render(&data, &opts);
        }

        // create file
        let mut dst = File::create(path)?;
        dst.write(data.as_bytes())?;

        // close file
        drop(dst);
      }
    }

    Ok(())
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
    // check if excluded
    for item in items.iter() {
      if item == &name {
        return true;
      }
    }
  
    return false;
  }

  fn create_meta(&self, target: &Path) -> Result<(), std::io::Error> {
    // create .tmpo.yaml file
    let meta_path = &target.to_path_buf().join(".tmpo.yaml");
    let mut meta_file = fs::File::create(meta_path)?;

    // fill meta
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
  // Run before script if exists
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

  let status = cmd.wait();
}


