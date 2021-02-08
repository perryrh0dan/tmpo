use std::collections::HashMap;
use std::env;
use std::fs;
use std::process::exit;

use crate::action::Action;
use crate::cli::input;
use crate::config;
use crate::context;
use crate::git;
use crate::out;
use crate::repository::{CopyOptions};
use crate::template::renderer;
use crate::utils;

use clap::ArgMatches;

impl Action {
  pub fn init(&self, args: &ArgMatches) {
    let ctx = context::Context::new(args);

    let workspace_name = args.value_of("name");
    let repository_name = args.value_of("repository");
    let template_name = args.value_of("template");
    let workspace_directory = args.value_of("directory");
    let remote_url = args.value_of("remote");
    let username = args.value_of("username");
    let email = args.value_of("email");

    out::info::initiate_workspace();

    // Check if repositories exist
    if self.config.get_repository_names().len() <= 0 {
      out::error::no_repositories();
      exit(1);
    }

    // Get workspace name form user input
    let workspace_name = if workspace_name.is_none() {
      match input::text("Please enter the project name", false) {
        Ok(value) => value,
        Err(error) => {
          log::error!("{}", error);
          eprintln!("{}", error);
          exit(1);
        }
      }
    } else {
      utils::lowercase(workspace_name.unwrap())
    };

    // Get repository
    let repository = match self.get_repository(repository_name) {
      Ok(repository) => repository,
      Err(error) => {
        log::error!("{}", error);
        eprintln!("{}", error);
        exit(1)
      }
    };

    // Check if templates exist
    let templates = repository.get_template_names();
    if templates.len() <= 0 {
      out::error::no_templates(&repository.get_config().name);
      exit(1);
    }

    let template_name = if template_name.is_none() {
      match input::select("template", &templates) {
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

    // Check if template exist
    if !templates.contains(&template_name) {
      out::error::template_not_found();
      exit(1);
    }

    // Get workspace directory from user input
    let workspace_directory = if workspace_directory.is_none() && !ctx.yes {
      match input::text_with_default("Please enter the target directory", &workspace_name) {
        Ok(value) => value,
        Err(error) => {
          log::error!("{}", error);
          eprintln!("{}", error);
          exit(1);
        }
      }
    } else if workspace_directory.is_some() {
      workspace_directory.unwrap().to_string()
    } else {
      String::from(".")
    };

    // Get target directory
    let current_dir = match env::current_dir() {
      Ok(dir) => dir,
      Err(error) => {
        log::error!("{}", error);
        eprintln!("{}", error);
        exit(1);
      }
    };

    // TODO find better solution
    // try to avoid . in path
    let target_dir = if workspace_directory != "." && workspace_directory != "./" {
      current_dir.join(workspace_directory)
    } else {
      current_dir
    };

    // Check if directory already exits
    if target_dir.exists() {
      log::error!("Failed to create workspace!: Error: Already exists");
      eprintln!("Failed to create workspace!: Error: Already exists");
      exit(1);
    }

    // Get workspace git repository url from user input
    let workspace_repository = if remote_url.is_none() && !ctx.yes {
      match input::text("Please enter a git remote url", true) {
        Ok(value) => value,
        Err(error) => {
          log::error!("{}", error);
          eprintln!("{}", error);
          exit(1);
        }
      }
    } else if remote_url.is_some() {
      remote_url.unwrap().to_string()
    } else {
      String::from("")
    };

    // Get email from user input or global git config
    let email = if email.is_none() && !ctx.yes {
      let git_email = match git::utils::get_email() {
        Ok(value) => value,
        Err(error) => {
          log::error!("{}", error);
          String::from("")
        }
      };

      match input::text_with_default(
        "Please enter your email",
        &git_email,
      ) {
        Ok(value) => value,
        Err(error) => {
          log::error!("{}", error);
          eprintln!("{}", error);
          exit(1);
        }
      }
    } else if email.is_some() {
      email.unwrap().to_owned()
    } else {
      String::from("")
    };

    // Get username from user input or global git config
    let username = if username.is_none() && !ctx.yes {
      let git_username = match git::utils::get_username() {
        Ok(value) => value,
        Err(error) => {
          log::error!("{}", error);
          String::from("")
        }
      };

      match input::text_with_default(
        "Please enter your username",
        &git_username,
      ) {
        Ok(value) => value,
        Err(error) => {
          log::error!("{}", error);
          eprintln!("{}", error);
          exit(1);
        }
      }
    } else if username.is_some() {
      username.unwrap().to_owned()
    } else {
      String::from("")
    };

    let mut values = HashMap::new();
    if !ctx.yes {
      // Get template specific values
      let keys = match repository.get_template_values(&template_name) {
        Ok(keys) => keys,
        Err(error) => {
          log::error!("{}", error);
          println!("{}", error);
          exit(1);
        }
      };

      for key in keys {
        let value = match input::text(&format!("Please enter {}", &key), true) {
          Ok(value) => value,
          Err(error) => {
            log::error!("{}", error);
            String::from("")
          }
        };
        values.insert(key, value);
      }
    }

    let tmp_dir = tempfile::Builder::new().tempdir_in(&config::temp_dir()).unwrap();

    // Create the temporary workspace
    let tmp_workspace_path = tmp_dir.path().join(&workspace_name);
    match fs::create_dir(&tmp_workspace_path) {
      Ok(()) => (),
      Err(error) => {
        log::error!("{}", error);
        eprintln!("{}", error);
        exit(1);
      }
    };

    // Initialize git if repository is given
    // Done here so that the repository can be used in the scripts
    if workspace_repository != "" {
      match git::init(&tmp_workspace_path, &workspace_repository) {
        Ok(()) => (),
        Err(error) => {
          log::error!("{}", error);
          eprintln!("{}", error);
          exit(1);
        }
      }
    }

    // Create context for renderer with custom values
    let render_context = renderer::Context {
      name: String::from(&workspace_name),
      repository: String::from(&workspace_repository),
      username: username,
      email: email,
      values: values,
    };

    // Create copy options
    let copy_options = CopyOptions {
      template_name: template_name.to_owned(),
      target: tmp_workspace_path.to_owned(),
      render_context: render_context,
    };

    // Copy the template
    log::info!("Start processing template: {}", &template_name);
    match repository.copy_template(&ctx, &copy_options) {
      Ok(()) => (),
      Err(error) => {
        log::error!("{}", error);
        eprintln!("{}", error);
        exit(1);
      }
    };

    // Create parent directories if they dont´t exist
    match fs::create_dir_all(&target_dir) {
      Ok(()) => (),
      Err(error) => {
        log::error!("{}", error);
        eprintln!("{}", error);
        exit(1);
      },
    };

    // Move workspace from temporary directroy to target directory
    log::info!(
      "Move workspace from: {} to: {}",
      tmp_workspace_path.to_string_lossy(),
      target_dir.to_string_lossy()
    );
    match std::fs::rename(tmp_workspace_path, target_dir) {
      Ok(()) => (),
      Err(error) => {
        log::error!("{}", error);
        eprintln!("{}", error);
        exit(1);
      }
    };

    out::success::workspace_created(&workspace_name);
  }
}
