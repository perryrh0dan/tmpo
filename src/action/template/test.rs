use std::process::exit;

use crate::action::Action;
use crate::cli::input;
use crate::out;
use crate::template;

use clap::ArgMatches;

impl Action {
  pub fn template_test(&self, args: &ArgMatches) {
    let directory = args.get_one::<String>("directory");

    // Get directory from user input
    let directory = if directory.is_none() {
      match input::text("Enter the target diectory", false) {
        Ok(value) => value,
        Err(error) => {
          log::error!("{}", error);
          eprintln!("{}", error);
          exit(1);
        }
      }
    } else {
      directory.unwrap().to_string()
    };

    let directory_path = std::path::Path::new(&directory);

    let template = match template::Template::new(directory_path) {
      Ok(template) => template,
      Err(error) => {
        log::error!("{}", error);
        eprintln!("{}", error);
        exit(1);
      }
    };

    out::info::display_template(&template);

    out::success::template_tested();
  }
}
