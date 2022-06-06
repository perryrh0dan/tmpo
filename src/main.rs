use log;
use std::process::exit;
use std::thread;

mod action;
mod app;
mod cli;
mod config;
mod context;
mod error;
mod git;
mod logger;
mod meta;
mod migration;
mod out;
mod renderer;
mod repository;
mod template;
mod update;
mod utils;
// TODO check for autocompletion for bash/zsh/powershell
// use clap_generate::{generate, generators::Bash};

fn main() {
  // Initiate logger
  logger::init();

  // migration
  match migration::check() {
    Ok(()) => (),
    Err(error) => {
      log::error!("{}", error);
    }
  };

  // Initiate config
  let config = match config::init() {
    Ok(data) => data,
    Err(error) => {
      log::error!("{}", error);
      eprintln!("{}", error);
      exit(1);
    }
  };

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

  let action = action::Action::new(config);

  // We build the command tree in a separate thread to eliminate
  // possible stack overflow crashes at runtime. OSX, for instance,
  // will crash with our large tree. This is a known issue:
  // https://github.com/kbknapp/clap-rs/issues/86
  let child = thread::Builder::new()
    .stack_size(8 * 1024 * 1024)
    .spawn(move || {
      let app = app::build();

      return app.get_matches()
    })
    .unwrap();

  let app_matches = child.join().unwrap();

  match app_matches.subcommand() {
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
        Some(("add", args)) => action.template_add(args),
        Some(("create", args)) => action.template_create(args),
        Some(("list", args)) => {
          action.template_list(args);
        }
        Some(("remove", args)) => {
          action.template_remove(args);
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
