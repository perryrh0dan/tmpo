use std::process::exit;

use crate::config;
use crate::cli::input;
use crate::out;
use crate::utils;

use clap::ArgMatches;

pub fn create(config: &mut config::Config, args: &ArgMatches) {
  let repository_name = args.value_of("repository");

  // Get repository name from user input
  let repository_name = if repository_name.is_none() {
    match input::text("repository name", false) {
      Some(value) => value,
      None => return,
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
}
