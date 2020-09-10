use crate::action;
use crate::config::Config;
use crate::out;

use clap::ArgMatches;

pub fn view(config: &Config, args: &ArgMatches) {
  let repository_name = args.value_of("repository");

  // Get repository
  let repository = match action::get_repository(&config, repository_name) {
    Some(value) => value,
    None => return,
  };

  out::info::display_repository(&repository);
}
