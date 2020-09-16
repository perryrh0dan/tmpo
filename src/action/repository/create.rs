use std::io::{Write};
use std::process::exit;

use crate::config::{Config, RepositoryOptions};
use crate::cli::input;
use crate::git;
use crate::meta;
use crate::out;
use crate::repository::{Repository};
use crate::utils;

use clap::ArgMatches;

pub fn create(config: &mut Config, args: &ArgMatches) {
  let name = args.value_of("repository");
  let description = args.value_of("description");
  let directory = args.value_of("directory");

  let repository_type = match input::select("Repository type", &vec!{String::from("remote"), String::from("local")}) {
    Ok(value) => value,
    Err(error) => {
      log::error!("{}", error);
      eprintln!("{}", error);
      exit(1);
    }
  };

  // Get repository name from user input
  let name = if name.is_none() {
    match input::text("repository name", false) {
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
  let repositories = config.get_repositories();
  if repositories.contains(&name) {
    out::error::repository_exists(&name);
    exit(1);
  }

  // Get repository name from user input
  let description = if description.is_none() {
    match input::text("repository description", false) {
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

  if repository_type == "remote" {
    create_remote(&name, &description, directory);
  } else {
    create_local(config, &name, &description);
  }
}

fn create_local(config: &mut Config, name: &str, description: &str) {
  config.template_repositories.push(RepositoryOptions {
    name: name.to_owned(),
    description: description.to_owned(),
    git_options: git::Options::new(),
  });

  let repository = match Repository::new(config, name) {
    Ok(repo) => repo,
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
      exit(1);
    }
  }

  out::success::local_repository_created(&name, &repository.directory.to_string_lossy());
}

fn create_remote(name: &str, description: &str, directory: Option<&str>) {
  // Get directory from user input
  let dir = if directory.is_none() {
    match input::text("Enter the target diectory", false) {
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

  // Create directory
  let repository_path = std::path::Path::new(&dir).join(name);
  match std::fs::create_dir(&repository_path) {
    Ok(_) => (),
    Err(error) => {
      log::error!("{}", error);
      eprintln!("{}", error);
      exit(1);
    }
  }

  // Create meta
  let meta_path = repository_path.join("meta.json");
  let mut meta_file = match std::fs::File::create(meta_path) {
    Ok(file) => file,
    Err(error) => {
      log::error!("{}", error);
      eprintln!("{}", error);
      exit(1);
    },
  };

  // Create meta data
  let mut meta = meta::Meta::new(meta::Type::REPOSITORY);
  meta.name = name.to_owned();
  meta.description = Some(description.to_owned());
  meta.version = Some(String::from("1.0.0"));

  let meta_data = serde_json::to_string_pretty(&meta).unwrap();
  match meta_file.write(meta_data.as_bytes()) {
    Ok(file) => file,
    Err(error) => {
      log::error!("{}", error);
      eprintln!("{}", error);
      exit(1);
    }
  };

  out::success::remote_repository_created(&name, &repository_path.to_string_lossy());
}
