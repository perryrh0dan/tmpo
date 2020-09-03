use std::io::ErrorKind;

use crate::cli::input::select;
use crate::config::Config;
use crate::out;
use crate::repository::Repository;
use crate::utils;

use clap::ArgMatches;

pub fn list(config: &Config, args: &ArgMatches) {
  let repository_name = args.value_of("repository");

  //// Get repository name from user input
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
  let mut repository = match Repository::new(config, &repository_name) {
    Ok(repository) => repository,
    Err(_error) => return,
  };

  repository.init();

  let templates = repository.get_templates();

  out::info::list_templates(&templates);
}
