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
    .about("Initialize new project")
    .subcommand(App::new("init")
      .about("Initialize new project")
      .arg(Arg::with_name("name")
        .help("name of the project")
        .required(false)
        .index(1))
      .arg(Arg::with_name("template")
        .help("template to use")
        .required(false)
        .index(2))
      .arg(Arg::with_name("dir")
        .help("Project directory")
        .required(false)
        .index(3)))
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