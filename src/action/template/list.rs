use std::process::exit;

use crate::action::Action;
use crate::out;

use clap::ArgMatches;

impl Action {
  pub fn template_list(&self, args: &ArgMatches) {
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

    let templates = repository.get_template_names();

    out::info::list_templates(&templates);
  }
}
