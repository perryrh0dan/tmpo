use std::fs;
use std::io::ErrorKind;
use std::path::{PathBuf};

use crate::cli::{input, select};
use crate::config::Config;
use crate::git;
use crate::renderer;
use crate::repository::{Repository, RepositoryError};
use crate::template;

use clap::ArgMatches;

pub fn init(config: &Config, args: &ArgMatches) {
  let workspace_name = args.value_of("name");
  let repository_name = args.value_of("repository");
  let template_name = args.value_of("template");
  let workspace_directory = args.value_of("directory");

  renderer::initiate_workspace();

  //// Get workspace name form user input
  let workspace_name = if workspace_name.is_none() {
    match input("Please enter the project name", false) {
      Some(value) => value,
      None => return,
    }
  } else {
    workspace_name.unwrap().to_string()
  };

  //// Get repository name from user input
  let repository_name = if repository_name.is_none() {
    let repositories = Repository::get_repositories(config);
    match select("repository", &repositories) {
      Ok(value) => value,
      Err(error) => match error.kind() {
        ErrorKind::InvalidData => {
          renderer::errors::no_repositories();
          return;
        },
        _ => std::process::exit(130),
      },
    }
  } else {
    String::from(repository_name.unwrap())
  };

  // Load repository
  let repository = match Repository::new(config, &repository_name) {
    Ok(repository) => repository,
    Err(error) => match error {
      RepositoryError::NotFound => return renderer::errors::repository_not_found(&repository_name),
      _ => return,
    },
  };

  //// Get template name from user input
  let template_name = if template_name.is_none() {
    let templates = repository.get_templates();
    match select("template", &templates) {
      Ok(value) => value,
      Err(error) => match error.kind() {
        ErrorKind::InvalidData => {
          renderer::errors::no_templates();
          return;
        },
        _ => return,
      },
    }
  } else {
    String::from(template_name.unwrap())
  };

  //// Get workspace directory from user input
  let workspace_directory = if workspace_directory.is_none() {
    match input("Please enter the target diectory", false) {
      Some(value) => value,
      None => return,
    }
  } else {
    workspace_directory.unwrap().to_string()
  };

  //// Get workspace git repository url from user input
  let workspace_repository = match input("Please enter a git remote url", true) {
    Some(value) => value,
    None => return,
  };

  //// Create the workspace directory
  let mut dir = PathBuf::from(workspace_directory);
  dir.push(&workspace_name);

  match fs::create_dir(&dir) {
    Ok(()) => (),
    Err(error) => match error.kind() {
      std::io::ErrorKind::AlreadyExists => (),
      _ => {
        renderer::errors::create_directory(&dir.to_string_lossy());
        return;
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

  //// Get the template
  let template = match repository.get_template_by_name(&template_name) {
    Ok(template) => template,
    Err(_error) => {
      renderer::errors::template_not_found(&template_name);
      return;
    }
  };

  let options = template::Options {
    name: String::from(&workspace_name),
    repository: Some(String::from(&workspace_repository)),
    username: username,
    email: email,
    replace: false,
  };

  //// Copy the template
  match template.copy(&repository, &dir, options) {
    Ok(()) => (),
    Err(_error) => {
      renderer::errors::copy_template();
      return;
    }
  };

  // Initialize git if repository is given
  if workspace_repository != "" {
    match git::init(&dir, &workspace_repository) {
      Ok(()) => (),
      Err(_error) => {
        renderer::errors::init_repository();
        return;
      }
    }
  }

  renderer::success_create(&workspace_name);
}
