use std::io::ErrorKind;

use crate::cli::input::select;
use crate::config::Config;
use crate::out;
use crate::repository::{Repository, RepositoryError};
use crate::utils;

use clap::ArgMatches;

pub fn view(config: &Config, args: &ArgMatches) {
  let repository_name = args.value_of("repository");
  let template_name = args.value_of("template");

  // Get repository name from user input
  let repository_name = if repository_name.is_none() {
    let repositories = config.get_repositories();
    match select("repository", &repositories) {
      Ok(value) => value,
      Err(error) => match error.kind() {
        ErrorKind::InvalidData => {
          out::error::no_repositories();
          return;
        },
        _ => return,
      },
    }
  } else {
    utils::lowercase(repository_name.unwrap())
  };

  // Load repository
  let repository = match Repository::new(config, &repository_name) {
    Ok(repository) => repository,
    Err(error) => {
      log::error!("{}", error);
      match error {
        RepositoryError::NotFound => out::error::repository_not_found(&repository_name),
        _ => out::error::unknown(),
      }
      return;
    },
  };

  // Get template name from user input
  let template_name = if template_name.is_none() {
    let templates = repository.get_templates();
    match select("template", &templates) {
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

  // Get the template
  let template = match repository.get_template_by_name(&template_name) {
    Ok(template) => template,
    Err(_error) => {
      out::error::template_not_found(&template_name);
      return;
    }
  };

  out::info::display_template(template);
}
