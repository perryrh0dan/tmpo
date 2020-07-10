mod action;
mod cli;
mod config;
mod git;
mod renderer;
mod repository;
mod template;
mod utils;

#[macro_use]
extern crate log;
extern crate clap;
use clap::{crate_version, App, AppSettings, Arg};
extern crate env_logger;

fn main() {
  // initialize logger
  env_logger::init();

  let matches = App::new("charon")
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
          Arg::with_name("replace")
            .long("replace")
            .help("When true, existing files are replaced")
            .required(false),
        ),
    )
    .subcommand(
      App::new("list")
        .about("List all available templates")
        .visible_alias("ls"),
    )
    .subcommand(App::new("update").about("Update to the latest release"))
    .subcommand(
      App::new("view")
        .about("View template details")
        .visible_alias("v")
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
            .help("The name of the template to use for generation")
            .required(false)
            .index(1),
        ),
    )
    .subcommand(
      App::new("repository")
        .about("Maintain repositories")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(App::new("add").about("Add new repository"))
        .subcommand(App::new("remove").about("Remove a repository")),
    )
    .get_matches();

  let mut config = config::init().unwrap();

  match matches.subcommand() {
    ("init", Some(init_matches)) => {
      action::init::init(&config, init_matches);
    }
    ("list", Some(list_matches)) => {
      action::list::list(&config, list_matches);
    }
    ("update", Some(_update_matches)) => {
      action::update::update();
    }
    ("view", Some(view_matches)) => {
      action::view::view(&config, view_matches);
    }
    ("repository", Some(repository_matches)) => {
      match repository_matches.subcommand() {
        ("add", Some(repo_add_matches)) => action::repository::add(&mut config, repo_add_matches),
        ("remove", Some(repo_delete_matches)) => {
          action::repository::remove(&mut config, repo_delete_matches)
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachabe!()
      }
    }
    _ => unreachable!(), // If all subcommands are defined above, anything else is unreachabe!()
  }
}
