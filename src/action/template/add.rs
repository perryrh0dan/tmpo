use std::process::exit;

use crate::action::Action;
use crate::config::TemplateOptions;
use crate::cli::input;
use crate::git;
use crate::meta;
use crate::out;
use crate::template;
use crate::utils;

use clap::ArgMatches;

impl Action {
  pub fn template_add(&self, args: &ArgMatches) {
    let template_name = args.value_of("name");
    let template_description = args.value_of("description");
    let template_url = args.value_of("repository");

    let mut git_options = git::Options::new();

    // Enable remote
    git_options.enabled = true;

    // Get provider
    git_options.provider = match input::select(
      "Provider",
      &vec![String::from("github"), String::from("gitlab")],
    ) {
      Ok(value) => {
        if value == "github" {
          Some(git::Provider::GITHUB)
        } else if value == "gitlab" {
          Some(git::Provider::GITLAB)
        } else {
          exit(1);
        }
      }
      Err(error) => {
        log::error!("{}", error);
        eprintln!("{}", error);
        exit(1)
      }
    };

    // Get authentication type
    git_options.auth = match input::select(
      "Auth type",
      &vec![
        String::from("token"),
        String::from("basic"),
        String::from("none"),
      ],
    ) {
      Ok(value) => {
        if value == "basic" {
          Some(git::AuthType::BASIC)
        } else if value == "none" {
          Some(git::AuthType::NONE)
        } else if value == "token" {
          Some(git::AuthType::TOKEN)
        } else {
          exit(1)
        }
      }
      Err(error) => {
        log::error!("{}", error);
        eprintln!("{}", error);
        exit(1)
      }
    };

    // Get template remote url
    git_options.url = if template_url.is_none() {
      match input::text("Enter remote template url", false) {
        Ok(value) => Some(value),
        Err(error) => {
          log::error!("{}", error);
          eprintln!("{}", error);
          exit(1);
        }
      }
    } else {
      Some(utils::lowercase(&template_url.unwrap()))
    };

    // Get branch
    git_options.branch = match input::text_with_default("Enter remote branch", "master") {
      Ok(value) => Some(value),
      Err(error) => {
        log::error!("{}", error);
        eprintln!("{}", error);
        exit(1);
      }
    };

    // Get credentials for different auth types
    match git_options.auth.clone().unwrap() {
      git::AuthType::BASIC => {
        git_options.username = match input::text("Enter your git username", false) {
          Ok(value) => Some(value),
          Err(error) => {
            log::error!("{}", error);
            eprintln!("{}", error);
            exit(1);
          }
        };
        git_options.password = match input::password("Enter your git password") {
          Ok(value) => Some(value),
          Err(error) => {
            log::error!("{}", error);
            eprintln!("{}", error);
            exit(1);
          }
        }
      }
      git::AuthType::SSH => {
        git_options.token = match input::text("Enter your git username", false) {
          Ok(value) => Some(value),
          Err(error) => {
            log::error!("{}", error);
            eprintln!("{}", error);
            exit(1);
          }
        }
      }
      git::AuthType::TOKEN => {
        git_options.token = match input::text("Enter your access token", false) {
          Ok(value) => Some(value),
          Err(error) => {
            log::error!("{}", error);
            eprintln!("{}", error);
            exit(1);
          }
        }
      }
      git::AuthType::NONE => {
        log::info!("[git]: no authentication");
      }
    }

    // Try to fetch meta data
    let meta = match meta::fetch(&git_options) {
      Ok(data) => data,
      Err(error) => {
        log::error!("{}", error);
        meta::Meta::new(meta::Type::REPOSITORY)
      }
    };

    // TODO validate meta kind

    // Get repository name from user input
    let template_name = if template_name.is_none() {
      match input::text_with_default("Enter repository name", &meta.name) {
        Ok(value) => value,
        Err(error) => {
          log::error!("{}", error);
          eprintln!("{}", error);
          exit(1);
        }
      }
    } else {
      utils::lowercase(template_name.unwrap())
    };

    // Validate name
    let templates = self.config.get_templates();
    if templates.contains(&template_name) {
      out::error::repository_exists(&template_name);
      exit(1);
    }

    // Get repository description from user input
    let template_description = if template_description.is_none() {
      let description = meta.description.unwrap_or_default();
      match input::text_with_default("Enter repository description", &description) {
        Ok(value) => value,
        Err(error) => {
          log::error!("{}", error);
          eprintln!("{}", error);
          exit(1);
        }
      }
    } else {
      template_description.unwrap().to_owned()
    };

    let options = TemplateOptions {
      name: template_name.to_owned(),
      description: Some(template_description),
      git_options: git_options,
    };

    // Add repository
    let new_config = match template::add(&self.config, options) {
      Ok(config) => config,
      Err(error) => {
        log::error!("{}", error);
        eprintln!("{}", error);
        exit(1)
      }
    };

    match new_config.save() {
      Ok(()) => (),
      Err(error) => {
        log::error!("{}", error);
        eprintln!("{}", error);
        exit(1)
      }
    }
  }
}
