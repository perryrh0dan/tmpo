use std::path::Path;
use std::process::exit;

use crate::action::Action;
use crate::config::{Config, RepositoryOptions};
use crate::cli::input;
use crate::git;
use crate::out;
use crate::repository::{Repository};
use crate::utils;

use clap::ArgMatches;

impl Action {
  pub fn repository_create(&self, args: &ArgMatches) {
    let name = args.value_of("name");
    let description = args.value_of("description");
    let directory = args.value_of("directory");
    let remote = args.value_of("remote");

    let repository_type = match input::select("Select a repository type", &vec!{String::from("remote"), String::from("local")}) {
      Ok(value) => value,
      Err(error) => {
        log::error!("{}", error);
        eprintln!("{}", error);
        exit(1);
      }
    };

    // Get repository name from user input
    let name = if name.is_none() {
      match input::text("Enter the repository name", false) {
        Ok(value) => value,
          Err(error) => {
            log::error!("{}", error);
            eprintln!("{}", error);
            exit(1);
          },
      }
    } else {
      utils::lowercase(name.unwrap())
    };

    // validate name
    let repositories = self.config.get_repositories();
    if repositories.contains(&name) {
      out::error::repository_exists(&name);
      exit(1);
    }

    // Get repository name from user input
    let description = if description.is_none() {
      match input::text("Enter the repository description", false) {
        Ok(value) => value,
        Err(error) => {
          log::error!("{}", error);
          eprintln!("{}", error);
          exit(1);
        },
      }
    } else {
      utils::lowercase(description.unwrap())
    };

    let mut options = RepositoryOptions{
      name: name.to_owned(),
      description: Some(description),
      git_options: git::Options::new(),
    };

    if repository_type == "remote" {
      create_remote(&self.config, &mut options, directory, remote);
    } else {
      create_local(&self.config, options);
    }
  }
}

fn create_local(config: &Config, options: RepositoryOptions) {
  let mut new_config = config.clone();
  new_config.template_repositories.push(options.clone());

  match Repository::create(Path::new(&new_config.template_dir), &options) {
    Ok(()) => (),
    Err(error) => {
      log::error!("{}", error);
      eprintln!("{}", error);
      exit(1);
    }
  };

  match new_config.save() {
    Ok(()) => (),
    Err(error) => {
      log::error!("{}", error);
      eprintln!("{}", error);
      exit(1);
    }
  }

  out::success::local_repository_created(&options.name, &new_config.template_dir);
}

fn create_remote(config: &Config, options: &mut RepositoryOptions, directory: Option<&str>, remote: Option<&str>) {
  // Get directory from user input
  let directory = if directory.is_none() {
    match input::text("Enter the target directory", false) {
      Ok(value) => value,
      Err(error) => {
        log::error!("{}", error);
        eprintln!("{}", error);
        exit(1);
      },
    }
  } else {
    directory.unwrap().to_string()
  };

  // Get remote from user input
  let remote = if remote.is_none() {
    match input::text("Enter the remote url", false) {
      Ok(value) => value,
      Err(error) => {
        log::error!("{}", error);
        eprintln!("{}", error);
        exit(1);
      },
    }
  } else {
    remote.unwrap().to_string()
  };

  options.git_options.url = Some(remote);

  // Create repository
  match Repository::create(&Path::new(&directory), options) {
    Ok(()) => (),
    Err(error) => {
      log::error!("{}", error);
      eprintln!("{}", error);
      exit(1);
    }
  };

  out::success::remote_repository_created(&options.name, &config.template_dir);
}
