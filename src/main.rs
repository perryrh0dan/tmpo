pub mod git;
pub mod config;
pub mod template;
pub mod renderer;
pub mod core;
mod cli;

extern crate clap;
use clap::{Arg, App};

extern crate ctrlc;

fn main() {
  match ctrlc::set_handler(move || {
    std::process::exit(130);
  }) {
    Ok(()) => (),
    Err(_e) => (),
  };

  let matches = App::new("Init")
    .version("0.1")
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
        match cli::init(&config, init_matches) {
          Ok(fc) => fc,
          Err(error) => println!("Error occured: {}", error)
        }
      }
      ("list", Some(_list_matches)) => {
        match core::list(&config) {
          Ok(fc) => fc,
          Err(error) => println!("Error occured: {}", error)
        };
      }
      ("", None) => println!("No subcommand was used"), // If no subcommand was usd it'll match the tuple ("", None)
      _ => unreachable!(), // If all subcommands are defined above, anything else is unreachabe!()
    }
}