use std::path::Path;
use std::process::exit;

use crate::action::Action;
use crate::cli::input;
use crate::config::RepositoryOptions;
use crate::context::Context;
use crate::meta;
use crate::meta::Type;
use crate::out;
use crate::utils;

use clap::ArgMatches;

impl Action {
  pub fn repository_add_external(&self, args: &ArgMatches) {
    let ctx = Context::new(args);

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

    let meta = match meta::load::<meta::RepositoryMeta>(&Path::new(&directory)) {
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
      directory: Some(directory),
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
