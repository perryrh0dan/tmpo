pub mod default;
pub mod repository;
pub mod template;

use crate::cli::input;
use crate::config;
use crate::out;
use crate::repository::{Repository, RepositoryError};

/// Validat given repository name or open a new selection
pub fn get_repository(
  config: &config::Config,
  repository_name: Option<&str>,
) -> Option<Repository> {
  // Get repository name from user input
  let repository_name = if repository_name.is_none() {
    let repositories = config.get_repositories();
    match input::select("repository", &repositories) {
      Ok(value) => value,
      Err(error) => {
        log::error!("{}", error);
        out::error::unknown();
        return None;
      },
    }
  } else {
    String::from(repository_name.unwrap())
  };

  // Load repository
  let repository = match Repository::new(config, &repository_name) {
    Ok(repository) => repository,
    Err(error) => {
      match error {
        RepositoryError::NotFound => out::error::repository_not_found(&repository_name),
        _ => out::error::unknown(),
      };
      return None;
    }
  };

  return Some(repository);
}
