use std::io::ErrorKind;
use std::process::exit;

use crate::action;
use crate::cli::input;
use crate::config::Config;
use crate::out;

use clap::ArgMatches;

pub fn remove(config: &mut Config, args: &ArgMatches) {
  let repository_name = args.value_of("repository");

  // Get repository
  let repository = match action::get_repository(&config, repository_name) {
    Ok(repository) => repository,
    Err(error) => {
      log::error!("{}", error);
      eprintln!("{}", error);
      exit(1)
    },
  };

  // Confirm
  let text = format!(
    "Are you sure you want to delete this item: {} (y/n): ",
    repository.config.name
  );
  if !input::confirm(&text) {
    return;
  }

  // remove template folder
  match repository.delete_repository() {
    Ok(()) => (),
    Err(error) => match error.kind() {
      ErrorKind::NotFound => (),
      _ => {
        out::error::unknown();
        return;
      }
    },
  };

  // Update config
  let index = config
    .template_repositories
    .iter()
    .position(|x| x.name == repository.config.name)
    .unwrap();
  config.template_repositories.remove(index);

  match config.save() {
    Ok(()) => (),
    Err(_error) => return,
  }

  out::success::repository_removed(&repository.config.name);
}
