use std::collections::HashMap;
use std::fs;
use std::path::{PathBuf};
use std::process::exit;

use crate::action;
use crate::cli::input;
use crate::config::Config;
use crate::git;
use crate::out;
use crate::template;
use crate::utils;

use clap::ArgMatches;

pub fn init(config: &Config, args: &ArgMatches) {
  let workspace_name = args.value_of("name");
  let repository_name = args.value_of("repository");
  let template_name = args.value_of("template");
  let workspace_directory = args.value_of("directory");
  let remote_url = args.value_of("remote");
  let username = args.value_of("username");
  let email = args.value_of("email");

  out::info::initiate_workspace();

  // check if repositories exist
  if config.get_repositories().len() <= 0 {
    out::error::no_repositories();
    exit(1);
  }

  // Get workspace name form user input
  let workspace_name = if workspace_name.is_none() {
    match input::text("Please enter the project name", false) {
      Ok(value) => value,
      Err(error) => {
        log::error!("{}", error);
        eprintln!("{}", error);
        exit(1);
      },
    }
  } else {
    utils::lowercase(workspace_name.unwrap())
  };

  // Get repository
  let repository = match action::get_repository(&config, repository_name) {
    Ok(repository) => repository,
    Err(error) => {
      log::error!("{}", error);
      eprintln!("{}", error);
      exit(1)
    },
  };

  // Check if templates exist
  if repository.get_templates().len() <= 0 {
    eprintln!("No templates exist in repository: {}", repository.config.name);
    exit(1);
  }

  let template_name = if template_name.is_none() {
    let templates = repository.get_templates();
    match input::select("template", &templates) {
      Ok(value) => value,
      Err(error) => {
        log::error!("{}", error);
        eprintln!("{}", error);
        exit(1);
      },
    }
  } else {
    String::from(template_name.unwrap())
  };

  // Get the template
  let template = match repository.get_template_by_name(&template_name) {
    Ok(template) => template,
    Err(error) => {
      eprintln!("{}", error);
      exit(1);
    }
  };

  // Get workspace directory from user input
  let workspace_directory = if workspace_directory.is_none() {
    match input::text("Please enter the target diectory", false) {
      Ok(value) => value,
      Err(error) => {
        log::error!("{}", error);
        eprintln!("{}", error);
        exit(1);
      },
    }
  } else {
    workspace_directory.unwrap().to_string()
  };

  // Get workspace git repository url from user input
  let workspace_repository = if remote_url.is_none() {
    match input::text("Please enter a git remote url", true) {
      Ok(value) => value,
      Err(error) => {
        log::error!("{}", error);
        eprintln!("{}", error);
        exit(1);
      },
    }
  } else {
    remote_url.unwrap().to_string()
  };

  // Create the workspace directory
  let mut dir = PathBuf::from(workspace_directory);
  dir.push(&workspace_name);

  match fs::create_dir(&dir) {
    Ok(()) => (),
    Err(error) => {
      log::error!("{}", error);
      eprintln!("{}", error);
      exit(1);
    }
  };

  // Get email from user input or global git config
  let email = if email.is_none() {
    let git_email = match git::get_email() {
      Ok(value) => value,
      Err(error) => {
        log::error!("{}", error);
        String::from("")
      },
    };

    match input::text(&format!("Please enter your email ({}): ", &git_email), true) {
      Ok(value) => Some(value),
      Err(error) => {
        log::error!("{}", error);
        eprintln!("{}", error);
        exit(1);
      },
    }
  } else {
    Some(email.unwrap().to_owned())
  };


  // Get username from user input or global git config
  let username = if username.is_none() {
    let git_username = match git::get_username() {
      Ok(value) => value,
      Err(error) => {
        log::error!("{}", error);
        String::from("")
      },
    };

    match input::text(&format!("Please enter your username ({}): ", &git_username), true) {
      Ok(value) => Some(value),
      Err(error) => {
        log::error!("{}", error);
        eprintln!("{}", error);
        exit(1);
      },
    }
  } else {
    Some(username.unwrap().to_owned())
  };

  // Get template specific values
  let mut values = HashMap::new();
  let keys = template.get_custom_values(&repository);
  for key in keys {
    let value = match input::text(&format!("Please enter {}", &key), true) {
      Ok(value) => value,
      Err(error) => {
        log::error!("{}", error);
        String::from("")
      },
    };
    values.insert(key, value);
  }

  let options = template::context::Context {
    name: String::from(&workspace_name),
    repository: Some(String::from(&workspace_repository)),
    username: username,
    email: email,
    values: values,
  };

  // Copy the template
  match template.copy(&repository, &dir, &options) {
    Ok(()) => (),
    Err(error) => {
      log::error!("{}", error);
      eprintln!("{}", error);
      exit(1);
    }
  };

  // Initialize git if repository is given
  if workspace_repository != "" {
    match git::init(&dir, &workspace_repository) {
      Ok(()) => (),
      Err(error) => {
        log::error!("{}", error);
        eprintln!("{}", error);
        exit(1);
      }
    }
  }

  out::success::workspace_created(&workspace_name);
}
