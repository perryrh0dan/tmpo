use std::fs;
use std::process::exit;

use crate::action::Action;
use crate::cli::input;
use crate::config::RepositoryOptions;
use crate::context::Context;
use crate::git;
use crate::meta;
use crate::meta::Type;
use crate::out;
use crate::repository::remote_repository;
use crate::utils;

use clap::ArgMatches;

impl Action {
  pub fn repository_add(&self, args: &ArgMatches) {
    let ctx = Context::new(args);

    let kind = args.value_of("type");

    let kind = if kind.is_none() {
      match input::select(
        "Type",
        &vec![String::from("remote"), String::from("directory")],
      ) {
        Ok(value) => value,
        Err(error) => {
          log::error!("{}", error);
          eprintln!("{}", error);
          exit(1)
        }
      }
    } else {
      String::from(kind.unwrap())
    };

    if kind == "remote" {
      self.repository_add_remote(&ctx, args);
    } else if kind == "directory" {
      self.repository_add_external(&ctx, args);
    }
  }

  pub fn repository_add_remote(&self, ctx: &Context, args: &ArgMatches) {
    let repository_name = args.value_of("name");
    let repository_description = args.value_of("description");
    let repository_provider = args.value_of("provider");
    let repository_authentication = args.value_of("authentication");
    let repository_url = args.value_of("url");
    let repository_branch = args.value_of("branch");
    let username = args.value_of("username");
    let password = args.value_of("password");
    let token = args.value_of("token");

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
      match input::text_with_default(&ctx, "Enter remote branch", "master") {
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
        // Get username
        git_options.username = if username.is_none() {
          match input::text("Enter your git username", false) {
            Ok(value) => Some(value),
            Err(error) => {
              log::error!("{}", error);
              eprintln!("{}", error);
              exit(1);
            }
          }
        } else {
          Some(String::from(username.unwrap()))
        };
        // Get password
        git_options.password = if password.is_none() {
          match input::password("Enter your git password") {
            Ok(value) => Some(value),
            Err(error) => {
              log::error!("{}", error);
              eprintln!("{}", error);
              exit(1);
            }
          }
        } else {
          Some(String::from(password.unwrap()))
        }
      }
      git::AuthType::SSH => {
        // Get ssh private key
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
        // Get token
        git_options.token = if token.is_none() {
          match input::text("Enter your access token", false) {
            Ok(value) => Some(value),
            Err(error) => {
              log::error!("{}", error);
              eprintln!("{}", error);
              exit(1);
            }
          }
        } else {
          Some(String::from(token.unwrap()))
        }
      }
      git::AuthType::NONE => {
        log::info!("[git]: no authentication");
      }
    }

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
      match input::text_with_default(&ctx, "Enter repository name", &meta.name) {
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
      match input::text_with_default(&ctx, "Enter repository description", &description) {
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
      kind: Some(String::from("remote")),
      directory: None,
      description: Some(repository_description),
      git_options: Some(git_options),
    };

    let mut new_config = self.config.clone();
    new_config.repositories.push(options.clone());

    // Add repository
    match remote_repository::add(&new_config, &options) {
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

  pub fn repository_add_external(&self, ctx: &Context, args: &ArgMatches) {
    let repository_name = args.value_of("name");
    let repository_description = args.value_of("description");
    let repository_directory = args.value_of("directory");

    // Get repository directory
    let directory = if repository_directory.is_none() {
      match input::text("Enter repository directory", false) {
        Ok(value) => value,
        Err(error) => {
          log::error!("{}", error);
          eprintln!("{}", error);
          exit(1);
        }
      }
    } else {
      String::from(repository_directory.unwrap())
    };

    let absolute_directory = match fs::canonicalize(&directory) {
      Ok(path) => path,
      Err(error) => {
        log::error!("{}", error);
        eprintln!("Wrong path");
        exit(1);
      }
    };

    let meta = match meta::load::<meta::RepositoryMeta>(&absolute_directory) {
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
      match input::text_with_default(&ctx, "Enter repository name", &meta.name) {
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
      match input::text_with_default(&ctx, "Enter repository description", &description) {
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
      kind: Some(String::from("external")),
      directory: Some(absolute_directory.as_path().display().to_string()),
      description: Some(repository_description),
      git_options: None,
    };

    let mut new_config = self.config.clone();
    new_config.repositories.push(options.clone());

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


