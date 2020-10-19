use std::process::exit;

use crate::action::Action;
use crate::cli::input;
use crate::out;
use crate::repository::custom_repository::CustomRepository;

use clap::ArgMatches;

impl Action {
  pub fn repository_remove(&self, args: &ArgMatches) {
    let repository_name = args.value_of("repository");

    // Get repository
    let repository_name = if repository_name.is_none() {
      let repositories = self.config.get_custom_repositories();
      input::select("repository", &repositories).unwrap()
    } else {
      String::from(repository_name.unwrap())
    };

    // Load repository
    let repository = if repository_name == "template" {
      CustomRepository::new(&self.config, &repository_name).unwrap()
    } else {
      CustomRepository::new(&self.config, &repository_name).unwrap()
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
