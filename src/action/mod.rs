mod config;
mod init;
mod repository;
mod template;
mod update;

use crate::cli::input;
use crate::config::Config;
use crate::error::RunError;
use crate::repository::Repository;
use crate::repository::remote_repository::RemoteRepository;
use crate::repository::default_repository::DefaultRepository;
use crate::repository::external_repository::ExternalRepository;

pub struct Action {
  config: Config,
}

impl Action {
  pub fn new(config: Config) -> Action {
    let act = Action { config: config };

    return act;
  }

  /// Validate given repository name or open a new selection
  fn get_repository(&self, repository_name: Option<&String>) -> Result<Box<dyn Repository>, RunError> {
    // Get repository name from user input
    let repository_name = if repository_name.is_none() {
      let repositories = self.config.get_repository_names();
      input::select("repository", &repositories)?
    } else {
      String::from(repository_name.unwrap())
    };

    if repository_name == "templates" {
      let repository = DefaultRepository::new(&self.config)?;
      return Ok(Box::new(repository));
    }

    let config = match self.config.get_repository_config(&repository_name) {
      Some(config) => config,
      None => return Err(RunError::Repository(String::from("Not found"))),
    };

    // Load repository
    let repository: Box<dyn Repository> = if config.kind == Some(String::from("external")) {
      let repository = ExternalRepository::new(&self.config, &repository_name)?;
      Box::new(repository)
    } else {
      let repository = RemoteRepository::new(&self.config, &repository_name)?;
      Box::new(repository)
    };

    Ok(repository)
  }
}
