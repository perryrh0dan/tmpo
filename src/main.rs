pub mod git;
pub mod config;
pub mod template;
pub mod renderer;
pub mod core;
mod cli;

extern crate clap;
use clap::{Arg, App, crate_version};

extern crate ctrlc;

fn main() {
  // catch ctrl + c
  match ctrlc::set_handler(move || {
    println!();
    std::process::exit(130);
  }) {
    Ok(()) => (),
    Err(_e) => (),
  };

  let matches = App::new("Init")
    .version(crate_version!())
    .author("Thomas P. <thomaspoehlmann96@googlemail.com>")
    .about("Cli to create new workspaces based on templates")
    .subcommand(App::new("init")
      .about("Initialize new project")
      .arg(Arg::with_name("name")
        .help("The name of the new workspace and initial project.")
        .required(true)
        .index(1))
      .arg(Arg::with_name("template")
        .short('t')
        .long("template")
        .takes_value(true)
        .help("The name of the template to use for generation")
        .required(false))
      .arg(Arg::with_name("directory")
        .short('d')
        .long("directory")
        .takes_value(true)
        .help("The directory name to create the workspace in.")
        .required(false))
      .arg(Arg::with_name("repository")
        .short('r')
        .long("repository")
        .takes_value(true)
        .help("The remote url to initialize the git repository")
        .required(false))
      .arg(Arg::with_name("replace")
        .long("replace")
        .help("When true, existing files are replaced")
        .required(false)))
    .subcommand(App::new("list")
      .about("List all available templates"))
    .get_matches();

    let config = config::init().unwrap();

    match matches.subcommand() {
      ("init", Some(init_matches)) => {
        cli::init(&config, init_matches);
      }
      ("list", Some(_list_matches)) => {
        cli::list(&config);
      }
      ("", None) => println!("No subcommand was used"), // If no subcommand was usd it'll match the tuple ("", None)
      _ => unreachable!(), // If all subcommands are defined above, anything else is unreachabe!()
    }
}