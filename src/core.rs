use std::fs;
use std::path::Path;

use crate::config::Config;
use crate::git;
use crate::renderer;
use crate::repository;

extern crate custom_error;
use custom_error::custom_error;

pub struct InitOpts {
  pub name: String,
  pub template: String,
  pub directory: String,
  pub repository: Option<String>,
  pub replace: bool,
}

custom_error! {pub CoreError
    CreateDirFailur      = "Unable to create workspace directory",
    CopyTemplateFailure  = "Unable to copy template",
    LoadRepository = "Unable to create repository",
    LoadTemplates        = "Unable to load templates",
    TemplateNotFound     = "Unable to find template",
    GitError             = "unable to initialize git repository"
}

/// Initialize a new Workspace
pub fn init(config: &Config, verbose: bool, opts: InitOpts) -> Result<(), CoreError> {
  renderer::initiate_workspace(&opts.name);

  let repository = match repository::Repository::new(config, verbose) {
    Ok(repository) => repository,
    Err(_error) => return Err(CoreError::LoadRepository)
  };

  //Create the workspace directory
  let dir = opts.directory + "/" + &opts.name;
  match fs::create_dir(Path::new(&dir)) {
    Ok(()) => (),
    Err(error) => match error.kind() {
      std::io::ErrorKind::AlreadyExists => (),
      _ => {
        renderer::errors::create_directory(&dir);
        return Err(CoreError::CreateDirFailur);
      }
    },
  };

  let mut email: Option<String> = None;
  match git::get_email() {
    Ok(value) => email = Some(value),
    Err(_error) => (),
  };

  let mut username: Option<String> = None;
  match git::get_username() {
    Ok(value) => username = Some(value),
    Err(_error) => (),
  };

  let options = repository::template::Options {
    name: String::from(&opts.name),
    repository: opts.repository.clone(),
    username: username,
    email: email,
    replace: opts.replace,
  };

  // copy the template
  let template = match repository.get_template_by_name(&opts.template) {
    Ok(template) => template,
    Err(_error) => {
      renderer::errors::template_not_found(&opts.template);
      return Err(CoreError::TemplateNotFound)
    }
  };

  match template.copy(&repository, &dir, options) {
    Ok(()) => (),
    Err(_error) => {
      renderer::errors::copy_template();
      return Err(CoreError::CopyTemplateFailure);
    }
  };

  // Initialize git if repository is given
  if !opts.repository.is_none() {
    match git::init(&dir, &opts.repository.unwrap()) {
      Ok(()) => (),
      Err(_error) => {
        renderer::errors::init_repository();
        return Err(CoreError::GitError);
      }
    }
  }

  renderer::success_create(&opts.name);

  Ok(())
}

/// List all available templates
pub fn list(config: &Config, verbose: bool) -> Result<(), CoreError> {
  let repository = match repository::Repository::new(config, verbose) {
    Ok(repository) => repository,
    Err(_error) => return Err(CoreError::LoadRepository)
  };

  let mut names = Vec::new();
  for template in repository.templates {
    names.push(template.name);
  }

  renderer::list_templates(&names);

  Ok(())
}

/// View details of a template
pub fn view(config: &Config, verbose: bool, name: &String) -> Result<(), CoreError> {
  let repository = match repository::Repository::new(config, verbose) {
    Ok(repository) => repository,
    Err(_error) => return Err(CoreError::LoadRepository)
  };

  let template = repository.get_template_by_name(name).unwrap();

  renderer::display_template(template);

  return Ok(())
}
