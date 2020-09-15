use std::process::exit;

use crate::cli::input;
use crate::config::{Config, RepositoryOptions};
use crate::git;
use crate::out;
use crate::repository::{Repository};
use crate::utils;

use clap::ArgMatches;

pub fn add(config: &mut Config, args: &ArgMatches) {
  let repository_name = args.value_of("repository");

  let mut git_options = git::GitOptions::new();

  // Get repository remote url
  git_options.url = match input::text("Please enter the remote repository url", false) {
    Ok(value) => Some(value),
    Err(error) => {
      log::error!("{}", error);
      eprintln!("{}", error);
      exit(1);
    },
  };

  // Try to fetch meta data


  // Get repository name from user input
  let repository_name = if repository_name.is_none() {
    match input::text("repository name", false) {
      Ok(value) => value,
      Err(error) => {
        log::error!("{}", error);
        eprintln!("{}", error);
        exit(1);
      },
    }
  } else {
    utils::lowercase(repository_name.unwrap())
  };

  // validate name
  let repositories = config.get_repositories();
  if repositories.contains(&repository_name) {
    out::error::repository_exists(&repository_name);
    exit(1);
  }

  // Get repository description from user input
  let repository_description = match input::text("repository description", false) {
    Ok(value) => value,
    Err(error) => {
      log::error!("{}", error);
      eprintln!("{}", error);
      exit(1);
    },
  };

  // Enable remote
  git_options.enabled = true;

  // Get provider
  git_options.provider = match input::select(
    "Provider",
    &vec![
      String::from("github"),
      String::from("gitlab"),
    ]
  ) {
    Ok(value) => Some(value),
    Err(error) => {
      log::error!("{}", error);
      eprintln!("{}", error);
      exit(1)
    }
  };

  // Get authentication type
  git_options.auth = match input::select(
    "Auth type",
    &vec![
      String::from("token"),
      String::from("basic"),
      String::from("none"),
    ],
  ) {
    Ok(value) => Some(value),
    Err(error) => {
      log::error!("{}", error);
      eprintln!("{}", error);
      exit(1)
    },
  };

  // Get credentials for different auth types
  if git_options.auth.clone().as_ref().unwrap() == "basic" {
    git_options.username = match input::text("Please enter your git username", false) {
      Ok(value) => Some(value),
      Err(error) => {
        log::error!("{}", error);
        eprintln!("{}", error);
        exit(1);
      },
    };
    git_options.password = match input::password("Please enter your git password") {
      Ok(value) => Some(value),
      Err(error) => {
        log::error!("{}", error);
        eprintln!("{}", error);
        exit(1);
      },
    }
  } else if git_options.auth.as_ref().unwrap() == "ssh" {
    git_options.token = match input::text("Please enter your git username", false) {
      Ok(value) => Some(value),
      Err(error) => {
        log::error!("{}", error);
        eprintln!("{}", error);
        exit(1);
      },
    }
  } else if git_options.auth.as_ref().unwrap() == "token" {
    git_options.token = match input::text("Please enter your access token", false) {
      Ok(value) => Some(value),
      Err(error) => {
        log::error!("{}", error);
        eprintln!("{}", error);
        exit(1);
      },
    }
  }

  config.template_repositories.push(RepositoryOptions {
    name: repository_name.to_owned(),
    description: repository_description,
    git_options: git_options,
  });

  // Load repository
  let repository = match Repository::new(config, &repository_name) {
    Ok(repository) => repository,
    Err(error) => {
      log::error!("{}", error);
      eprintln!("{}", error);
      exit(1)
    }
  };

  // Test repository
  match repository.test() {
    Ok(()) => (),
    Err(error) => {
      log::error!("{}", error);
      eprintln!("{}", error);
      exit(1);
    }
  };

  match config.save() {
    Ok(()) => (),
    Err(error) => {
      log::error!("{}", error);
      eprintln!("{}", error);
      exit(1)
    },
  }

  out::success::repository_added(&repository_name);
}
