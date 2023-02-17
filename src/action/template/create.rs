use log;
use std::path::Path;
use std::process::exit;

use crate::action::Action;
use crate::cli::input;
use crate::config::Config;
use crate::context::Context;
use crate::meta;
use crate::out;
use crate::repository::Repository;
use crate::repository::default_repository::DefaultRepository;
use crate::template;

use clap::ArgMatches;

impl Action {
  pub fn template_create(&self, args: &ArgMatches) {
    let ctx = Context::new(args);

    let template_name = args.get_one::<String>("template");
    let directory = args.get_one::<String>("directory");

    // TODO create template in given directory
    let template_type = match input::select(
      "Template type",
      &vec![String::from("directory"), String::from("local")],
    ) {
      Ok(value) => value,
      Err(error) => {
        log::error!("{}", error);
        eprintln!("{}", error);
        exit(1);
      }
    };

    if template_type == "directory" {
      create_remote(&ctx, template_name, directory);
    } else {
      create_local(&self.config, template_name);
    }
  }
}

fn create_local(config: &Config, template_name: Option<&String>) {
  // Load repository
  let repository = match DefaultRepository::new(config) {
    Ok(repository) => repository,
    Err(error) => {
      log::error!("{}", error);
      eprintln!("{}", error);
      exit(1)
    }
  };

  // Create meta data
  let mut meta = meta::TemplateMeta::new(meta::Type::TEMPLATE);

  // Get template name from user input
  meta.name = if template_name.is_none() {
    match input::text("template name", false) {
      Ok(value) => value,
      Err(error) => {
        log::error!("{}", error);
        eprintln!("{}", error);
        exit(1);
      }
    }
  } else {
    String::from(template_name.unwrap())
  };

  // validate name
  if repository.get_template_names().contains(&meta.name) {
    out::error::template_exists(&meta.name);
    exit(1)
  }

  let repository_directory = repository.directory;

  let template_path = match template::create(&repository_directory, &meta) {
    Ok(value) => value,
    Err(error) => {
      log::error!("{}", error);
      println!("{}", error);
      exit(1)
    }
  };

  out::success::template_created(&template_path.to_str().unwrap());
}

fn create_remote(ctx: &Context, template_name: Option<&String>, directory: Option<&String>) {
  // Create meta data
  let mut meta = meta::TemplateMeta::new(meta::Type::TEMPLATE);

  // Get template name from user input
  meta.name = if template_name.is_none() {
    match input::text("Enter the template name", false) {
      Ok(value) => value,
      Err(error) => {
        log::error!("{}", error);
        eprintln!("{}", error);
        exit(1);
      }
    }
  } else {
    String::from(template_name.unwrap())
  };

  // Get template directory from user input
  let directory: String = if directory.is_none() {
    match input::text_with_default(&ctx, "Enter the target directory", ".") {
      Ok(value) => value,
      Err(error) => {
        log::error!("{}", error);
        eprintln!("{}", error);
        exit(1);
      }
    }
  } else {
    String::from(directory.unwrap())
  };

  let directory_path = Path::new(&directory);
  let template_path = match template::create(&directory_path, &meta) {
    Ok(value) => value,
    Err(error) => {
      log::error!("{}", error);
      println!("{}", error);
      exit(1)
    }
  };

  out::success::template_created(&template_path.to_str().unwrap());
}
