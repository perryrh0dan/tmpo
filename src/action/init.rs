use std::fs;
use std::path::Path;

use crate::config::Config;
use crate::git;
use crate::renderer;
use crate::repository;

use clap::ArgMatches;
use dialoguer::{theme::ColorfulTheme, Input, Select};

pub fn init(config: &Config, args: &ArgMatches) {
  let workspace_name = args.value_of("name");
  let template_name = args.value_of("template");
  let workspace_directory = args.value_of("directory");
  let workspace_repository = args.value_of("repository");

  ////
  renderer::initiate_workspace();

  //// Get workspace name form user input
  let workspace_name = if workspace_name.is_none() {
    match Input::<String>::new()
      .with_prompt("project name")
      .interact()
    {
      Ok(name) => name,
      Err(_error) => return,
    }
  } else {
    workspace_name.unwrap().to_string()
  };

  //// Get repository name from user input
  let repositories = repository::get_repositories(config);
  let selection = match dialoguer::Select::with_theme(&ColorfulTheme::default())
    .with_prompt("Select a repository")
    .default(0)
    .items(&repositories[..])
    .interact()
  {
    Ok(selection) => selection,
    Err(_error) => return,
  };

  let repository_name = String::from(&repositories[selection]);

  // Load repository
  let repository = match repository::Repository::new(config, &repository_name) {
    Ok(repository) => repository,
    Err(_error) => return,
  };

  //// Get template name from user input
  let template_name = if template_name.is_none() {
    let templates = repository.get_templates();
    let selection = match Select::with_theme(&ColorfulTheme::default())
      .with_prompt("Pick a template")
      .default(0)
      .items(&templates[..])
      .interact()
    {
      Ok(selection) => selection,
      Err(_error) => return,
    };
    String::from(&templates[selection])
  } else {
    String::from(template_name.unwrap())
  };

  //// Get workspace directory from user input
  let workspace_directory = if workspace_directory.is_none() {
    match Input::<String>::new()
      .with_prompt("target directory")
      .interact()
    {
      Ok(directory) => directory,
      Err(_error) => return,
    }
  } else {
    workspace_directory.unwrap().to_string()
  };

  //// Get workspace git repository url from user input
  let workspace_repository = if workspace_repository.is_none() {
    match Input::<String>::new()
      .with_prompt("repository url")
      .allow_empty(true)
      .interact()
    {
      Ok(repository) => repository,
      Err(_error) => return,
    }
  } else {
    workspace_repository.unwrap().to_string()
  };

  // Create the workspace directory
  let dir = workspace_directory + "/" + &workspace_name;
  match fs::create_dir(Path::new(&dir)) {
    Ok(()) => (),
    Err(error) => match error.kind() {
      std::io::ErrorKind::AlreadyExists => (),
      _ => {
        renderer::errors::create_directory(&dir);
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

  // Get the template
  let template = match repository.get_template_by_name(&template_name) {
    Ok(template) => template,
    Err(_error) => {
      renderer::errors::template_not_found(&template_name);
      return;
    }
  };

  let options = repository::template::Options {
    name: String::from(&template_name),
    repository: None, //workspace_repository
    username: username,
    email: email,
    replace: false,
  };

  // Copy the template
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
