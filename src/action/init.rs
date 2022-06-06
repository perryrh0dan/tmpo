use std::collections::HashMap;
use std::env;
use std::fs;
use std::process::exit;

use crate::action::Action;
use crate::cli::input;
use crate::config;
use crate::context;
use crate::git;
use crate::meta::TemplateType;
use crate::out;
use crate::renderer;
use crate::repository::CopyOptions;
use crate::utils;

use clap::ArgMatches;
use fs_extra::dir;

impl Action {
  pub fn init(&self, args: &ArgMatches) {
    let mut ctx = context::Context::new(args);

    ctx.set_no_script(args.is_present("no_script"));

    // Parse arguments
    let workspace_name = args.value_of("name");
    let repository_name = args.value_of("repository");
    let template_name = args.value_of("template");
    let workspace_directory = args.value_of("directory");

    out::info::initiate_workspace();

    // Check if repositories exist
    if self.config.get_repository_names().len() <= 0 {
      out::error::no_repositories();
      exit(1);
    }

    // Get workspace name form user input
    let workspace_name = if workspace_name.is_none() {
      match input::text("Please enter the project/snippet name", false) {
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

    let template = match repository.get_template_by_name(&template_name) {
      Ok(template) => template,
      Err(error) => {
        log::error!("{}", error);
        out::error::template_not_found();
        exit(1);
      }
    };

    // Get workspace directory from user input
    let workspace_directory = if workspace_directory.is_none() {
      match input::text_with_default(&ctx, "Please enter the target directory", &workspace_name) {
        Ok(value) => value,
        Err(error) => {
          log::error!("{}", error);
          eprintln!("{}", error);
          exit(1);
        }
      }
    } else {
      workspace_directory.unwrap().to_string()
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

    let mut render_context = if template.meta.sub_type == TemplateType::PROJECT {
      self.init_project(&ctx, &workspace_name, args)
    } else {
      self.init_snippet(&ctx, &workspace_name,args)
    };

    if !ctx.yes {
      // TODO think about
      // Get template specific values
      let values = match repository.get_template_values(&template_name) {
        Ok(keys) => keys,
        Err(error) => {
          log::error!("{}", error);
          println!("{}", error);
          exit(1);
        }
      };

      for value in values {
        let input = if value.default.is_some() {
          // Get and parse default value
          let default_value = renderer::render(&value.default.to_owned().unwrap(), &render_context);

          match input::text_with_default(
            &ctx,
            &format!("Please enter {}", value.get_label()),
            &default_value,
          ) {
            Ok(value) => value,
            Err(error) => {
              log::error!("{}", error);
              String::from("")
            }
          }
        } else {
          let required = if value.required.is_some() {
            value.required.to_owned().unwrap()
          } else {
            false
          };

          match input::text(&format!("Please enter {}", value.get_label()), !required) {
            Ok(value) => value,
            Err(error) => {
              log::error!("{}", error);
              String::from("")
            }
          }
        };

        // Update inputs map
        render_context.values.insert(value.key, input);

        // // Update render context to use new input
        // render_context.values = inputs.clone()
      }
    }

    let tmp_dir = tempfile::Builder::new()
      .tempdir_in(&config::temp_dir())
      .unwrap();

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
    if render_context.repository != "" {
      match git::init(&tmp_workspace_path, &render_context.repository) {
        Ok(()) => (),
        Err(error) => {
          log::error!("{}", error);
          eprintln!("{}", error);
          exit(1);
        }
      }
    }

    // Create copy options
    let copy_options = CopyOptions {
      template_name: template_name.to_owned(),
      target: tmp_workspace_path.to_owned(),
      render_context: render_context.to_owned(),
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
    let mut parent_dir = target_dir.to_owned();
    parent_dir.pop();
    match fs::create_dir_all(&parent_dir) {
      Ok(()) => (),
      Err(error) => {
        log::error!("{}", error);
        eprintln!("{}", error);
        exit(1);
      }
    };

    // Move workspace from temporary directory to target directory
    log::info!(
      "Move workspace from: {} to: {}",
      tmp_workspace_path.to_string_lossy(),
      target_dir.to_string_lossy()
    );

    // Create target directory
    match fs::create_dir_all(target_dir.clone()) {
      Ok(()) => (),
      Err(error) => {
        log::error!("{}", error);
        eprintln!("{}", error);
        exit(1);
      }
    }

    match dir::copy(tmp_workspace_path, target_dir, &dir::CopyOptions::new()) {
      Ok(_result) => (),
      Err(error) => {
        log::error!("{}", error);
        eprintln!("{}", error);
        exit(1);
      }
    };

    // Print success message
    out::success::workspace_created(&workspace_name);

    // Get Template info
    let template_info = match repository.get_template_info(&template_name) {
      Ok(info) => info,
      Err(error) => {
        log::error!("{}", error);
        eprintln!("{}", error);
        exit(1);
      }
    };

    // Print template info
    if template_info.is_some() {
      let info = renderer::render(&template_info.unwrap(), &render_context);
      out::success::workspace_info(&info);
    }
  }

  fn init_project(&self, ctx: &context::Context, workspace_name: &str, args: &ArgMatches) -> renderer::Context {
    let remote_url = args.value_of("remote");
    let username = args.value_of("username");
    let email = args.value_of("email");

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

      match input::text_with_default(&ctx, "Please enter your email", &git_email) {
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

      match input::text_with_default(&ctx, "Please enter your username", &git_username) {
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

    // Create context for renderer with custom values
    let render_context = renderer::Context {
      name: String::from(workspace_name),
      repository: String::from(&workspace_repository),
      username: username,
      email: email,
      values: HashMap::new(),
    };

    return render_context;
  }

  fn init_snippet(&self, _ctx: &context::Context, workspace_name: &str, _args: &ArgMatches) -> renderer::Context {
    // Create context for renderer with custom values
    let render_context = renderer::Context {
      name: String::from(workspace_name),
      repository: String::from(""),
      username: String::from(""),
      email: String::from(""),
      values: HashMap::new(),
    };

    return render_context;
  }
}
