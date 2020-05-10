mod cli;
pub mod config;
pub mod core;
pub mod git;
pub mod renderer;
pub mod template;
pub mod utils;

#[macro_use]
extern crate log;
extern crate clap;
use clap::{crate_version, App, Arg};
extern crate ctrlc;
extern crate env_logger;

fn main() {
  // catch ctrl + c
  match ctrlc::set_handler(move || {
    println!();
    std::process::exit(130);
  }) {
    Ok(()) => (),
    Err(_e) => (),
  };

  // initialize logger
  env_logger::init();

  let matches = App::new("charon")
    .version(crate_version!())
    .author("Thomas P. <thomaspoehlmann96@googlemail.com>")
    .about("Cli to create new workspaces based on templates")
    .arg(
      Arg::with_name("verbose")
        .short('v')
        .long("verbose")
        .help("When true, more verbose output is displayed")
        .required(false),
    )
    .subcommand(
      App::new("init")
        .about("Initialize new workspace")
        .arg(
          Arg::with_name("name")
            .help("The name of the new workspace and initial project.")
            .required(true)
            .index(1),
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
          Arg::with_name("repository")
            .short('r')
            .long("repository")
            .takes_value(true)
            .help("The remote url to initialize the git repository")
            .required(false),
        )
        .arg(
          Arg::with_name("replace")
            .long("replace")
            .help("When true, existing files are replaced")
            .required(false),
        ),
    )
    .subcommand(App::new("list").about("List all available templates"))
    .subcommand(App::new("update").about("Update to the latest release"))
    .get_matches();

  let verbose;
  match matches.occurrences_of("verbose") {
    0 => verbose = false,
    1 | _ => verbose = true,
  }

  let config = config::init(verbose).unwrap();

  match matches.subcommand() {
    ("init", Some(init_matches)) => {
      cli::init(&config, init_matches);
    }
    ("list", Some(_list_matches)) => {
      cli::list(&config);
    }
    ("update", Some(_update_matches)) => {
      println!("Update command not available yet");
    }
    ("", None) => renderer::warnings::no_subcommand(), // If no subcommand was usd it'll match the tuple ("", None)
    _ => unreachable!(), // If all subcommands are defined above, anything else is unreachabe!()
  }
}
