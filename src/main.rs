mod action;
mod cli;
mod config;
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
  // Initiate config
  let mut config = match config::init() {
    Ok(data) => data,
    Err(_error) => {
      out::error::load_config();
      std::process::exit(1)
    }
  };

  // Initiate logger
  logger::init();

  // TODO Check for update
  let matches = App::new("tmpo")
    .version(crate_version!())
    .author("Thomas P. <thomaspoehlmann96@googlemail.com>")
    .about("Cli to create new workspaces based on templates")
    .setting(AppSettings::ArgRequiredElseHelp)
    .subcommand(
      App::new("init")
        .about("Initialize new workspace")
        .visible_alias("i")
        .arg(
          Arg::with_name("name")
            .help("The name of the new workspace and initial project.")
            .required(false)
            .index(1),
        )
        .arg(
          Arg::with_name("repository")
            .short('r')
            .long("repository")
            .takes_value(true)
            .help("The repository to use")
            .required(false),
        )
        .arg(
          Arg::with_name("template")
            .short('t')
            .long("template")
            .takes_value(true)
            .help("The name of the template to use for generation")
            .required(false),
        )
        .arg(
          Arg::with_name("directory")
            .short('d')
            .long("directory")
            .takes_value(true)
            .help("The directory name to create the workspace in.")
            .required(false),
        )
        .arg(
          Arg::with_name("remote")
            .long("remote")
            .takes_value(true)
            .help("Remote URL")
            .required(false),
        )
        .arg(
          Arg::with_name("replace")
            .long("replace")
            .help("When true, existing files are replaced")
            .required(false),
        ),
    )
    .subcommand(
      App::new("config")
        .about("View configuration"),
    )
    .subcommand(App::new("update").about("Update to the latest release"))
    .subcommand(
      App::new("repository")
        .about("Maintain repositories")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(App::new("add").about("Add new repository"))
        .subcommand(App::new("list").about("List all available repository"))
        .subcommand(App::new("remove").about("Remove a repository")),
    )
    .subcommand(
      App::new("template")
        .about("Maintain templates")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(App::new("create").about("Create new template"))
        .subcommand(App::new("list").about("List all available templates"))
        .subcommand(
          App::new("view")
            .about("View template details")
            .arg(
              Arg::with_name("repository")
                .short('r')
                .long("repository")
                .takes_value(true)
                .help("Name of the repository")
                .required(false),
            )
            .arg(
              Arg::with_name("template")
                .short('t')
                .long("template")
                .takes_value(true)
                .help("Name of the template")
                .required(false)
            ),
        ),
    )
    .get_matches();

  match matches.subcommand() {
    ("config", Some(_config_matches)) => {
      action::default::config::config(&config);
    }
    ("init", Some(init_matches)) => {
      action::default::init::init(&config, init_matches);
    }
    ("update", Some(_update_matches)) => {
      action::default::update::update();
    }
    ("repository", Some(repository_matches)) => {
      match repository_matches.subcommand() {
        ("add", Some(repo_add_matches)) => {
          action::repository::add::add(&mut config, repo_add_matches)
        }
        ("remove", Some(repo_delete_matches)) => {
          action::repository::remove::remove(&mut config, repo_delete_matches)
        }
        ("list", Some(_list_matches)) => {
          action::repository::list::list(&config);
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachabe!()
      }
    }
    ("template", Some(repository_matches)) => {
      match repository_matches.subcommand() {
        ("create", Some(template_create_matches)) => {
          action::template::create::create(&mut config, template_create_matches)
        }
        ("view", Some(view_matches)) => {
          action::template::view::view(&config, view_matches);
        }
        ("list", Some(list_matches)) => {
          action::template::list::list(&config, list_matches);
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachabe!()
      }
    }
    _ => unreachable!(), // If all subcommands are defined above, anything else is unreachabe!()
  }
}
