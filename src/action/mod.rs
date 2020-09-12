pub mod default;
pub mod repository;
pub mod template;

use crate::cli::input;
use crate::config;
use crate::error::RunError;
use crate::repository::{Repository};

/// Validat given repository name or open a new selection
pub fn get_repository(
  config: &config::Config,
  repository_name: Option<&str>,
) -> Result<Repository, RunError> {
  // Get repository name from user input
  let repository_name = if repository_name.is_none() {
    let repositories = config.get_repositories();
    input::select("repository", &repositories)?
  } else {
    String::from(repository_name.unwrap())
  };

  // Load repository
  let repository = Repository::new(config, &repository_name)?;

  return Ok(repository);
}
