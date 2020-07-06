use crate::cli;

use crate::config::Config;
use crate::renderer;
use crate::repository;

extern crate clap;
use clap::ArgMatches;

pub fn view(config: &Config, args: &ArgMatches, verbose: bool) {
  let template_opt = args.value_of("template");

  // Get template
  let template: String;
  if template_opt.is_none() {
    template = match cli::get_value("template to show", true, None) {
      Ok(template) => template.unwrap(),
      Err(_error) => return,
    };
  } else {
    template = template_opt.unwrap().to_string();
  }

  let repository = match repository::Repository::new(config, verbose) {
    Ok(repository) => repository,
    Err(_error) => return,
  };

  let template = repository.get_template_by_name(&template).unwrap();

  renderer::display_template(template);
}
