use std::io::ErrorKind;

use crate::action;
use crate::cli::input::select;
use crate::config::Config;
use crate::out;
use clap::ArgMatches;

pub fn view(config: &Config, args: &ArgMatches) {
  let repository_name = args.value_of("repository");
  let template_name = args.value_of("template");

  // Get repository
  let repository = match action::get_repository(&config, repository_name) {
    Some(value) => value,
    None => return,
  };

  // Get template name from user input
  let template_name = if template_name.is_none() {
    let templates = repository.get_templates();
    match select("template", &templates) {
      Ok(value) => value,
      Err(error) => match error.kind() {
        ErrorKind::InvalidData => {
          out::error::no_templates(&repository.config.name);
          return;
        }
        _ => return,
      },
    }
  } else {
    String::from(template_name.unwrap())
  };

  // Get the template
  let template = match repository.get_template_by_name(&template_name) {
    Ok(template) => template,
    Err(_error) => {
      out::error::template_not_found(&template_name);
      return;
    }
  };

  out::info::display_template(template);
}
