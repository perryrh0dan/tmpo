use crate::config::Config;
use crate::renderer;
use crate::repository;

use dialoguer::{theme::ColorfulTheme, Select};

pub fn list(config: &Config) {
  //// Get repository name from user input
  let repositories = repository::get_repositories(config);
  let selection = match Select::with_theme(&ColorfulTheme::default())
    .with_prompt("Pick a repository")
    .default(0)
    .items(&repositories[..])
    .interact()
  {
    Ok(selection) => selection,
    Err(_error) => return,
  };
  let repository_name = String::from(&repositories[selection]);

  // Load repository
  let repository = match repository::Repository::new(config, &repository_name) {
    Ok(repository) => repository,
    Err(_error) => return,
  };

  let mut names = Vec::new();
  for template in repository.templates {
    names.push(template.name);
  }

  renderer::list_templates(&names);
}
