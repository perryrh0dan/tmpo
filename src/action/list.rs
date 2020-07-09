use std::io::ErrorKind;

use crate::cli::select;
use crate::config::Config;
use crate::renderer;
use crate::repository::Repository;

use clap::ArgMatches;

pub fn list(config: &Config, args: &ArgMatches) {
  let repository_name = args.value_of("repository");

  //// Get repository name from user input
  let repository_name = if repository_name.is_none() {
    let repositories = Repository::get_repositories(config);
    match select("repository", &repositories) {
      Ok(value) => value,
      Err(error) => match error.kind() {
        ErrorKind::InvalidData => {
          renderer::errors::no_repositories();
          return;
        },
        _ => return,
      },
    }
  } else {
    String::from(repository_name.unwrap())
  };

  // Load repository
  let repository = match Repository::new(config, &repository_name) {
    Ok(repository) => repository,
    Err(_error) => return,
  };

  let mut names = Vec::new();
  for template in repository.templates {
    names.push(template.name);
  }

  renderer::list_templates(&names);
}
