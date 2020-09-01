use std::fs;
use std::io::ErrorKind;
use std::path::{PathBuf};

use crate::cli::input;
use crate::config::Config;
use crate::git;
use crate::out;
use crate::repository::{Repository, RepositoryError};
use crate::template;

use clap::ArgMatches;

pub fn init(config: &Config, args: &ArgMatches) {
  let workspace_name = args.value_of("name");
  let repository_name = args.value_of("repository");
  let template_name = args.value_of("template");
  let workspace_directory = args.value_of("directory");
  let remote_url = args.value_of("remote");

  out::info::initiate_workspace();

  // check if repositories exist
  if config.get_repositories().len() <= 0 {
    out::error::no_repositories();
    return;
  }

  //// Get workspace name form user input
  let workspace_name = if workspace_name.is_none() {
    match input::text("Please enter the project name", false) {
      Some(value) => value,
      None => return,
    }
  } else {
    workspace_name.unwrap().to_string()
  };

  //// Get repository name from user input
  let repository_name = if repository_name.is_none() {
    let repositories = config.get_repositories();
    match input::select("repository", &repositories) {
      Ok(value) => value,
      Err(error) => {
        log::error!("{}", error);
        out::error::unknown();
        return;
      },
    }
  } else {
    String::from(repository_name.unwrap())
  };

  // Load repository
  let repository = match Repository::new(config, &repository_name) {
    Ok(repository) => repository,
    Err(error) => match error {
      RepositoryError::NotFound => return out::error::repository_not_found(&repository_name),
      _ => return,
    },
  };

  //// Get template name from user input
  let template_name = if template_name.is_none() {
    let templates = repository.get_templates();
    match input::select("template", &templates) {
      Ok(value) => value,
      Err(error) => match error.kind() {
        ErrorKind::InvalidData => {
          out::error::no_templates(&repository.config.name);
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
    match input::text("Please enter the target diectory", false) {
      Some(value) => value,
      None => return,
    }
  } else {
    workspace_directory.unwrap().to_string()
  };

  //// Get workspace git repository url from user input
  let workspace_repository = if remote_url.is_none() {
    match input::text("Please enter a git remote url", true) {
      Some(value) => value,
      None => return,
    }
  } else {
    remote_url.unwrap().to_string()
  };

  //// Create the workspace directory
  let mut dir = PathBuf::from(workspace_directory);
  dir.push(&workspace_name);

  match fs::create_dir(&dir) {
    Ok(()) => (),
    Err(error) => match error.kind() {
      std::io::ErrorKind::AlreadyExists => (),
      _ => {
        out::error::create_directory(&dir.to_string_lossy());
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
      out::error::template_not_found(&template_name);
      return;
    }
  };

  let options = template::context::Context {
    name: String::from(&workspace_name),
    repository: Some(String::from(&workspace_repository)),
    username: username,
    email: email,
  };

  //// Copy the template
  match template.copy(&repository, &dir, options) {
    Ok(()) => (),
    Err(_error) => {
      out::error::copy_template();
      return;
    }
  };

  // Initialize git if repository is given
  if workspace_repository != "" {
    match git::init(&dir, &workspace_repository) {
      Ok(()) => (),
      Err(_error) => {
        out::error::init_repository();
        return;
      }
    }
  }

  out::success::workspace_created(&workspace_name);
}
