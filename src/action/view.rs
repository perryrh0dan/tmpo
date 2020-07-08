use crate::config::Config;
use crate::renderer;
use crate::repository;

use clap::ArgMatches;
use dialoguer::{theme::ColorfulTheme, Select};

pub fn view(config: &Config, args: &ArgMatches) {
  let repository_name = args.value_of("repository");
  let template_name = args.value_of("template");

  //// Get repository name from user input
  let repository_name = if repository_name.is_none() {
    let repositories = repository::get_repositories(config);
    let selection = match dialoguer::Select::with_theme(&ColorfulTheme::default())
      .with_prompt("Select a repository")
      .default(0)
      .items(&repositories[..])
      .interact()
    {
      Ok(selection) => selection,
      Err(_error) => return,
    };
    String::from(&repositories[selection])
  } else {
    String::from(repository_name.unwrap())
  };

  // Load repository
  let repository = match repository::Repository::new(config, &repository_name) {
    Ok(repository) => repository,
    Err(_error) => return,
  };

  //// Get template name from user input
  let template_name = if template_name.is_none() {
    let templates = repository.get_templates();
    let selection = match Select::with_theme(&ColorfulTheme::default())
      .with_prompt("Pick a template")
      .default(0)
      .items(&templates[..])
      .interact()
    {
      Ok(selection) => selection,
      Err(_error) => return,
    };
    String::from(&templates[selection])
  } else {
    String::from(template_name.unwrap())
  };

  let template = repository.get_template_by_name(&template_name).unwrap();

  renderer::display_template(template);
}
