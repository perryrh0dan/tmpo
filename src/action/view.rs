use crate::config::Config;
use crate::renderer;
use crate::repository;

use clap::ArgMatches;
use dialoguer::{theme::ColorfulTheme, Select};

pub fn view(config: &Config, args: &ArgMatches) {
  let template_name = args.value_of("template");

  //// Get repository name from user input
  let repositories = repository::get_repositories(config);
  let selection = dialoguer::Select::with_theme(&ColorfulTheme::default())
    .with_prompt("Pick a repository")
    .default(0)
    .items(&repositories[..])
    .interact()
    .unwrap();
  let repository_name = String::from(&repositories[selection]);

  // Load repository
  let repository = match repository::Repository::new(config, &repository_name) {
    Ok(repository) => repository,
    Err(_error) => return,
  };

  //// Get template name from user input
  let template_name = if template_name.is_none() {
    let templates = repository.get_templates();
    let selection = Select::with_theme(&ColorfulTheme::default())
      .with_prompt("Pick a template")
      .default(0)
      .items(&templates[..])
      .interact()
      .unwrap();
    String::from(&templates[selection])
  } else {
    String::from(template_name.unwrap())
  };

  let template = repository.get_template_by_name(&template_name).unwrap();

  renderer::display_template(template);
}
