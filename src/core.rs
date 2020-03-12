use std::fs;
use std::path::{Path};

use crate::config::Config;
use crate::git; 
use crate::template;
use crate::renderer;

pub struct InitOpts {
  pub name: String,
  pub template: String,
  pub dir: String,
  pub repository: Option<String>,
}

extern crate custom_error;
use custom_error::custom_error;

custom_error!{pub CoreError
    CreateProjectDir = "Unable to create project directory",
    CopyTemplate     = "Unable to copy template",
    InitializeRepo   = "Unable to initialize git",
    LoadTemplates    = "Unable to load templates",
}

/// Initialize a new Project
pub fn init(config: &Config, opts: InitOpts) -> Result<(), CoreError> {
  //Create directory the project directory 
  let dir = opts.dir + "/" + &opts.name;
  match fs::create_dir(Path::new(&dir)) {
    Ok(fc) => fc,
    Err(_error) => {
      renderer::errors::create_directory(&dir);
      return Err(CoreError::CreateProjectDir);
    },
  };

  let copy_opts = template::CopyOpts {
    template: String::from(&opts.template),
    dir: String::from(&dir),
    name: String::from(&opts.name),
    repository: opts.repository.clone(),
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

  renderer::success_create();

  Ok(())
}

/// List all available templates
pub fn list(config: &Config) -> Result<(), CoreError> {
  let templates = match template::get_all_templates(config) {
    Ok(templates) => templates,
    Err(_error) => return Err(CoreError::LoadTemplates),
  };

  renderer::list_templates(&templates);

  Ok(())
}