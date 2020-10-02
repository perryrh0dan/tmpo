use log;
use std::process::exit;

mod action;
mod cli;
mod config;
mod context;
mod error;
mod git;
mod logger;
mod meta;
mod out;
mod repository;
mod template;
mod update;
mod utils;

use clap::{crate_version, App, AppSettings, Arg};

fn main() {
  // Initiate logger
  logger::init();

  // Initiate config
  let config = match config::init() {
    Ok(data) => data,
    Err(error) => {
      log::error!("{}", error);
      eprintln!("{}", error);
      exit(1);
    }
  };

  // Check if user has internet connection
  if utils::online() {
    // Check for an update
    match update::check_version() {
      Some((available_version, _asset)) => {
        let current_version = crate_version!();
        println!(
          "New release found! {} --> {}",
          current_version, available_version
        );
      }
      None => (),
    };
  }

  let action = action::Action::new(config);

  let matches = App::new("tmpo")
    .version(crate_version!())
    .global_setting(AppSettings::VersionlessSubcommands)
    .author("Thomas P. <thomaspoehlmann96@googlemail.com>")
    .about("Cli to create new workspaces based on templates")
    .setting(AppSettings::ArgRequiredElseHelp)
    .setting(AppSettings::HelpRequired)
    .subcommand(
      App::new("init")
        .about("Initialize new workspace")
        .visible_alias("i")
        .arg(
          Arg::new("name")
            .about("Name of the new workspace and the project.")
            .required(false)
            .index(1),
        )
        .arg(
          Arg::new("directory")
            .short('d')
            .long("directory")
            .takes_value(true)
            .about("Directory name to create the workspace in.")
            .required(false),
        )
        .arg(
          Arg::new("remote")
            .long("remote")
            .takes_value(true)
            .about("Remote URL")
            .required(false),
        )
        .arg(
          Arg::new("repository")
            .short('r')
            .long("repository")
            .takes_value(true)
            .about("Repository to use")
            .required(false),
        )
        .arg(
          Arg::new("template")
            .short('t')
            .long("template")
            .takes_value(true)
            .about("Template to use for generation")
            .required(false),
        )
        .arg(
          Arg::new("username")
            .long("username")
            .takes_value(true)
            .about("Username of the user")
            .required(false),
        )
        .arg(
          Arg::new("email")
            .long("email")
            .takes_value(true)
            .about("E-Mail of the user")
            .required(false),
        )
        .arg(
          Arg::new("yes")
            .short('y')
            .long("yes")
            .takes_value(false)
            .about("Skip all optional questions")
        )
        .arg(
          Arg::new("no-script")
            .long("no-script")
            .takes_value(false)
            .about("Dont execute template scripts")
            .required(false),
        ),
    )
    .subcommand(App::new("config").about("View configuration"))
    .subcommand(App::new("update").about("Update to the latest release"))
    .subcommand(
      App::new("repository")
        .about("Maintain repositories")
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::HelpRequired)
        .subcommand(
          App::new("add")
            .about("Add remote repository")
            .arg(
              Arg::new("name")
                .short('n')
                .long("name")
                .takes_value(true)
                .about("Name of the repository")
                .required(false),
            )
            .arg(
              Arg::new("description")
                .short('d')
                .long("description")
                .takes_value(true)
                .about("Description of the repository")
                .required(false),
            ),
        )
        .subcommand(
          App::new("create")
            .about("Create a new repository")
            .arg(
              Arg::new("name")
                .short('n')
                .long("name")
                .takes_value(true)
                .about("Name of the repository")
                .required(false),
            )
            .arg(
              Arg::new("description")
                .short('d')
                .long("description")
                .takes_value(true)
                .about("Description of the repository")
                .required(false),
            ),
        )
        .subcommand(App::new("list").about("List all available repository"))
        .subcommand(
          App::new("remove").about("Remove a repository").arg(
            Arg::new("repository")
              .short('r')
              .long("repository")
              .takes_value(true)
              .about("Name of the repository")
              .required(false),
          ),
        )
        .subcommand(
          App::new("view").about("View repository details").arg(
            Arg::new("repository")
              .short('r')
              .long("repository")
              .takes_value(true)
              .about("Name of the repository")
              .required(false),
          ),
        ),
    )
    .subcommand(
      App::new("template")
        .about("Maintain templates")
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::HelpRequired)
        .subcommand(
          App::new("create")
            .about("Create new template")
            .arg(
              Arg::new("repository")
                .short('r')
                .long("repository")
                .takes_value(true)
                .about("Name of the repository")
                .required(false),
            )
            .arg(
              Arg::new("name")
                .short('n')
                .long("name")
                .takes_value(true)
                .about("Name of the template")
                .required(false),
            ),
        )
        .subcommand(
          App::new("list").about("List all available templates").arg(
            Arg::new("repository")
              .short('r')
              .long("repository")
              .takes_value(true)
              .about("Name of the repository")
              .required(false),
          ),
        )
        .subcommand(
          App::new("test")
            .about("Test template at given location")
            .arg(
              Arg::new("directory")
                .short('d')
                .long("directory")
                .takes_value(true)
                .about("Directory of the template")
                .required(true),
            ),
        )
        .subcommand(
          App::new("view")
            .about("View template details")
            .arg(
              Arg::new("repository")
                .short('r')
                .long("repository")
                .takes_value(true)
                .about("Name of the repository")
                .required(false),
            )
            .arg(
              Arg::new("template")
                .short('t')
                .long("template")
                .takes_value(true)
                .about("Name of the template")
                .required(false),
            ),
        ),
    )
    .get_matches();

  match matches.subcommand() {
    Some(("config", _args)) => {
      action.config();
    }
    Some(("init", args)) => {
      action.init(args);
    }
    Some(("update", _args)) => {
      action.update();
    }
    Some(("repository", args)) => {
      match args.subcommand() {
        Some(("add", args)) => action.repository_add(args),
        Some(("create", args)) => action.repository_create(args),
        Some(("list", _args)) => {
          action.repository_list();
        }
        Some(("remove", args)) => action.repository_remove(args),
        Some(("view", args)) => action.repository_view(args),
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachabe!()
      }
    }
    Some(("template", args)) => {
      match args.subcommand() {
        Some(("create", args)) => action.template_create(args),
        Some(("list", args)) => {
          action.template_list(args);
        }
        Some(("test", args)) => {
          action.template_test(args);
        }
        Some(("view", args)) => {
          action.template_view(args);
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachabe!()
      }
    }
    _ => unreachable!(), // If all subcommands are defined above, anything else is unreachabe!()
  }
}
