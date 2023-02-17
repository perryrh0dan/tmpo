use std::process::exit;

use crate::action::Action;
use crate::out;

use clap::ArgMatches;

impl Action {
  pub fn repository_view(&self, args: &ArgMatches) {
    let repository_name = args.get_one::<String>("repository");

    // Get repository
    let repository = match self.get_repository(repository_name) {
      Ok(repository) => repository,
      Err(error) => {
        log::error!("{}", error);
        eprintln!("{}", error);
        exit(1)
      }
    };

    out::info::display_repository(repository);
  }
}
