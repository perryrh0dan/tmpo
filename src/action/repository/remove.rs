use std::process::exit;

use crate::action::Action;
use crate::cli::input;
use crate::out;

use clap::ArgMatches;

impl Action {
  pub fn repository_remove(&self, args: &ArgMatches) {
    let repository_name = args.value_of("repository");

    // Get repository
    let repository = match self.get_repository(repository_name) {
      Ok(repository) => repository,
      Err(error) => {
        log::error!("{}", error);
        eprintln!("{}", error);
        exit(1)
      }
    };

    // Confirm
    let text = format!(
      "Are you sure you want to delete this item: {} (y/n): ",
      repository.config.name
    );
    if !input::confirm(&text) {
      exit(0);
    }

    // Remove template folder
    match repository.delete_repository() {
      Ok(()) => (),
      Err(error) => {
        log::error! {"{}", error}
      }
    };

    // Update config
    let mut new_config = self.config.clone();
    let index = new_config
      .template_repositories
      .iter()
      .position(|x| x.name == repository.config.name)
      .unwrap();
    new_config.template_repositories.remove(index);

    match new_config.save() {
      Ok(()) => (),
      Err(error) => {
        log::error!("{}", error);
        eprintln!("{}", error);
        exit(1)
      }
    }

    out::success::repository_removed(&repository.config.name);
  }
}
