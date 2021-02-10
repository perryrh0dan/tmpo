use std::process::exit;

use crate::action::Action;
use crate::cli::input;
use crate::config::RepositoryOptions;
use crate::git;
use crate::meta;
use crate::meta::Type;
use crate::out;
use crate::repository::custom_repository;
use crate::utils;

use clap::ArgMatches;

impl Action {
  pub fn repository_add(&self, args: &ArgMatches) {
    let repository_name = args.value_of("name");
    let repository_description = args.value_of("description");
    let repository_provider = args.value_of("provider");
    let repository_authentication = args.value_of("authentication");
    let repository_url = args.value_of("url");
    let repository_branch = args.value_of("branch");

    let mut git_options = git::Options::new();

    // Enable remote
    git_options.enabled = true;

    // Get provider
    git_options.provider = if repository_provider.is_none() {
      match input::select(
        "Provider",
        &vec![String::from("github"), String::from("gitlab")],
      ) {
        Ok(value) => match git::Provider::from(&value) {
          Ok(provider) => Some(provider),
          Err(error) => {
            log::error!("{}", error);
            eprintln!("{}", error);
            exit(1)
          }
        },
        Err(error) => {
          log::error!("{}", error);
          eprintln!("{}", error);
          exit(1)
        }
      }
    } else {
      match git::Provider::from(&repository_provider.unwrap()) {
        Ok(provider) => Some(provider),
        Err(error) => {
          eprintln!("{}", error);
          exit(1)
        }
      }
    };

    // Get authentication type
    git_options.auth = if repository_authentication.is_none() {
      match input::select(
        "Auth type",
        &vec![
          String::from("token"),
          String::from("basic"),
          String::from("none"),
        ],
      ) {
        Ok(value) => match git::AuthType::from(&value) {
          Ok(auth_type) => Some(auth_type),
          Err(error) => {
            log::error!("{}", error);
            eprintln!("{}", error);
            exit(1)
          }
        },
        Err(error) => {
          log::error!("{}", error);
          eprintln!("{}", error);
          exit(1)
        }
      }
    } else {
      match git::AuthType::from(&repository_authentication.unwrap()) {
        Ok(auth_type) => Some(auth_type),
        Err(error) => {
          log::error!("{}", error);
          eprintln!("{}", error);
          exit(1)
        }
      }
    };

    // Get repository remote url
    git_options.url = if repository_url.is_none() {
      match input::text("Enter remote repository url", false) {
        Ok(value) => Some(value),
        Err(error) => {
          log::error!("{}", error);
          eprintln!("{}", error);
          exit(1);
        }
      }
    } else {
      Some(String::from(repository_url.unwrap()))
    };

    // Get branch
    git_options.branch = if repository_branch.is_none() {
      match input::text_with_default("Enter remote branch", "master") {
        Ok(value) => Some(value),
        Err(error) => {
          log::error!("{}", error);
          eprintln!("{}", error);
          exit(1);
        }
      }
    } else {
      Some(String::from(repository_branch.unwrap()))
    };

    // Get credentials for different auth types
    match git_options.auth.clone().unwrap() {
      git::AuthType::BASIC => {
        git_options.username = match input::text("Enter your git username", false) {
          Ok(value) => Some(value),
          Err(error) => {
            log::error!("{}", error);
            eprintln!("{}", error);
            exit(1);
          }
        };
        git_options.password = match input::password("Enter your git password") {
          Ok(value) => Some(value),
          Err(error) => {
            log::error!("{}", error);
            eprintln!("{}", error);
            exit(1);
          }
        }
      }
      git::AuthType::SSH => {
        git_options.token = match input::text("Enter your git username", false) {
          Ok(value) => Some(value),
          Err(error) => {
            log::error!("{}", error);
            eprintln!("{}", error);
            exit(1);
          }
        }
      }
      git::AuthType::TOKEN => {
        git_options.token = match input::text("Enter your access token", false) {
          Ok(value) => Some(value),
          Err(error) => {
            log::error!("{}", error);
            eprintln!("{}", error);
            exit(1);
          }
        }
      }
      git::AuthType::NONE => {
        log::info!("[git]: no authentication");
      }
    }

    // Try to fetch meta data
    let meta = match meta::fetch::<meta::RepositoryMeta>(&git_options) {
      Ok(data) => data,
      Err(error) => {
        log::error!("{}", error);
        meta::RepositoryMeta::new(meta::Type::REPOSITORY)
      }
    };

    // Check for meta type repository
    if meta.kind != Type::REPOSITORY {
      log::error!("{}", format!("Wrong type: {}", meta.kind));
      eprintln!("{}", format!("Wrong type: {}", meta.kind));
      exit(1)
    }

    // Get repository name from user input
    let repository_name = if repository_name.is_none() {
      match input::text_with_default("Enter repository name", &meta.name) {
        Ok(value) => value,
        Err(error) => {
          log::error!("{}", error);
          eprintln!("{}", error);
          exit(1);
        }
      }
    } else {
      utils::lowercase(repository_name.unwrap())
    };

    // Validate name
    let repositories = self.config.get_repository_names();
    if repositories.contains(&repository_name) {
      out::error::repository_exists(&repository_name);
      exit(1);
    }

    // Get repository description from user input
    let repository_description = if repository_description.is_none() {
      let description = meta.description.unwrap_or_default();
      match input::text_with_default("Enter repository description", &description) {
        Ok(value) => value,
        Err(error) => {
          log::error!("{}", error);
          eprintln!("{}", error);
          exit(1);
        }
      }
    } else {
      repository_description.unwrap().to_owned()
    };

    let options = RepositoryOptions {
      name: repository_name.to_owned(),
      description: Some(repository_description),
      git_options: git_options,
    };

    let mut new_config = self.config.clone();
    new_config.repositories.push(options.clone());

    // Add repository
    match custom_repository::add(&new_config, &options) {
      Ok(repository) => repository,
      Err(error) => {
        log::error!("{}", error);
        eprintln!("{}", error);
        exit(1)
      }
    };

    match new_config.save() {
      Ok(()) => (),
      Err(error) => {
        log::error!("{}", error);
        eprintln!("{}", error);
        exit(1)
      }
    }

    out::success::repository_added(&repository_name);
  }
}
