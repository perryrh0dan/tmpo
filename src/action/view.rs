use std::io::ErrorKind;

use crate::cli::select;
use crate::config::Config;
use crate::renderer;
use crate::repository::Repository;

use clap::ArgMatches;

pub fn view(config: &Config, args: &ArgMatches) {
  let repository_name = args.value_of("repository");
  let template_name = args.value_of("template");

  //// Get repository name from user input
  let repository_name = if repository_name.is_none() {
    let repositories = Repository::get_repositories(config);
    match select("repository", &repositories) {
      Ok(value) => value,
      Err(error) => match error.kind() {
        ErrorKind::InvalidData => {
          renderer::errors::no_repositories();
          return;
        }
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

  //// Get template name from user input
  let template_name = if template_name.is_none() {
    let templates = repository.get_templates();
    match select("template", &templates) {
      Ok(value) => value,
      Err(error) => match error.kind() {
        ErrorKind::InvalidData => {
          renderer::errors::no_templates();
          return;
        },
        _ => return,
      },
    }
  } else {
    String::from(template_name.unwrap())
  };

  let template = repository.get_template_by_name(&template_name).unwrap();

  renderer::display_template(template);
}
