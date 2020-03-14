use std::fs;
use std::path::{Path};

use crate::config::Config;
use crate::git; 
use crate::template;
use crate::renderer;

pub struct InitOpts {
  pub name: String,
  pub template: String,
  pub directory: String,
  pub repository: Option<String>,
  pub replace: bool,
}

extern crate custom_error;
use custom_error::custom_error;

custom_error!{pub CoreError
    CreateDir        = "Unable to create workspace directory",
    CopyTemplate     = "Unable to copy template",
    InitializeRepo   = "Unable to initialize git",
    LoadTemplates    = "Unable to load templates",
}

/// Initialize a new Workspace
pub fn init(config: &Config, opts: InitOpts) -> Result<(), CoreError> {
  //Create directory the workspace directory 
  let dir = opts.directory + "/" + &opts.name;
  match fs::create_dir(Path::new(&dir)) {
    Ok(()) => (),
    Err(error) => match error.kind() {
      std::io::ErrorKind::AlreadyExists => (),
      _ => {
        renderer::errors::create_directory(&dir);
        return Err(CoreError::CreateDir);
      }
    },
  };

  let copy_opts = template::Options {
    template: String::from(&opts.template),
    dir: String::from(&dir),
    name: String::from(&opts.name),
    repository: opts.repository.clone(),
    replace: opts.replace,
  };

  match template::copy(config, copy_opts){
    Ok(()) => (),
    Err(_error) => {
      renderer::errors::copy_template();
      return Err(CoreError::CopyTemplate);
    }
  };

  if !opts.repository.is_none() {
    match git::init(&dir, &opts.repository.unwrap()) {
      Ok(()) => (),
      Err(_error) => {
        renderer::errors::init_repository();
        return Err(CoreError::InitializeRepo);
      },
    }
  }

  renderer::success_create(&opts.name);

  Ok(())
}

/// List all available templates
pub fn list(config: &Config) -> Result<(), CoreError> {
  let templates = match template::get_all_templates(config) {
    Ok(templates) => templates,
    Err(_error) => return Err(CoreError::LoadTemplates),
  };

  let mut names = Vec::new();
  for template in templates {
    names.push(template.name);
  }

  renderer::list_templates(&names);

  Ok(())
}