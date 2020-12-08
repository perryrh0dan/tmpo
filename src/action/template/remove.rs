use std::process::exit;

use crate::action::Action;
use crate::cli::input;
use crate::out;

use crate::repository::{Repository, default_repository::DefaultRepository};

use clap::ArgMatches;

impl Action {
  pub fn template_remove(&self, args: &ArgMatches) {
    let template_name = args.value_of("template");

    // Load repository
    let repository = DefaultRepository::new(&self.config).unwrap();

    // Check if templates exist
    let templates = repository.get_template_names();
    if templates.len() <= 0 {
      eprintln!(
        "No templates exist in repository: {}",
        repository.get_config().name
      );
      exit(1);
    }

    // Get template name from user input
    let template_name = if template_name.is_none() {
      match input::select("template", &templates) {
        Ok(value) => value,
        Err(error) => {
          log::error!("{}", error);
          eprintln!("{}", error);
          exit(1)
        }
      }
    } else {
      String::from(template_name.unwrap())
    };

    // Validate name
    if repository.get_template_names().contains(&template_name) {
      out::error::template_exists(&template_name);
      exit(1);
    }

    // Confirm
    let text = format!(
      "Are you sure you want to delete this template: {} (y/n): ",
      template_name
    );
    if !input::confirm(&text) {
      exit(0);
    }

    // Remove template folder
    match repository.remove_template(&template_name) {
      Ok(()) => (),
      Err(error) => {
        log::error! {"{}", error}
      }
    }

    // Update config
    let mut new_config = self.config.clone();
    let index = new_config
      .templates
      .iter()
      .position(|x| x.name == template_name)
      .unwrap();
    new_config.repositories.remove(index);

    match new_config.save() {
      Ok(()) => (),
      Err(error) => {
        log::error!("{}", error);
        eprintln!("{}", error);
        exit(1)
      }
    }
  }
}
