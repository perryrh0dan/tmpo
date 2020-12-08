mod config;
mod init;
mod repository;
mod template;
mod update;

use crate::cli::input;
use crate::config::Config;
use crate::error::RunError;
use crate::repository::Repository;
use crate::repository::custom_repository::CustomRepository;
use crate::repository::default_repository::DefaultRepository;

pub struct Action {
  config: Config,
}

impl Action {
  pub fn new(config: Config) -> Action {
    let act = Action { config: config };

    return act;
  }

  /// Validat given repository name or open a new selection
  fn get_repository(&self, repository_name: Option<&str>) -> Result<Box<dyn Repository>, RunError> {
    // Get repository name from user input
    let repository_name = if repository_name.is_none() {
      let repositories = self.config.get_repository_names();
      input::select("repository", &repositories)?
    } else {
      String::from(repository_name.unwrap())
    };

    // Load repository
    let repository: Box<dyn Repository> = if repository_name == "templates" {
      let repository = DefaultRepository::new(&self.config)?;
      Box::new(repository)
    } else {
      let repository = CustomRepository::new(&self.config, &repository_name)?;
      Box::new(repository)
    };

    Ok(repository)
  }
}
