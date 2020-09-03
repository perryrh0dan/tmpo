use std::io::ErrorKind;

use crate::cli::input::select;
use crate::config::{Config};
use crate::out;
use crate::repository::{Repository, RepositoryError};
use crate::utils;

use clap::ArgMatches;

pub fn remove(config: &mut Config, args: &ArgMatches) {
    let repository_name = args.value_of("repository");

    // Get repository name from user input
    let repository_name = if repository_name.is_none() {
      let repositories = config.get_repositories();
      match select("repository", &repositories) {
        Ok(value) => value,
        Err(error) => match error.kind() {
          ErrorKind::InvalidData => {
            out::error::no_repositories();
            return;
          }
          _ => return,
        },
      }
    } else {
      utils::lowercase(repository_name.unwrap())
    };

    // Load repository
    let repository = match Repository::new(config, &repository_name) {
      Ok(repository) => repository,
      Err(error) => return match error {
          RepositoryError::NotFound => out::error::repository_not_found(&repository_name),
          _ => out::error::unknown(),
      },
    };

    // remove template folder
    match repository.delete_repository() {
        Ok(()) => (),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => (),
            _ => {
                out::error::unknown();
                return;
            },
        },
    };

    // Update config
    let index = config
        .template_repositories
        .iter()
        .position(|x| x.name == repository_name)
        .unwrap();
    config.template_repositories.remove(index);

    match config.save() {
        Ok(()) => (),
        Err(_error) => return,
    }

    out::success::repository_removed(&repository_name);
}
