// (Full example with detailed comments in examples/01a_quick_example.rs)
//
// This example demonstrates clap's "builder pattern" method of creating arguments
// which the most flexible, but also most verbose.
extern crate clap;
use clap::{Arg, App};

pub mod git;
pub mod config;
pub mod template;
mod core;

fn main() {
  let matches = App::new("Init")
    .version("0.1")
    .author("Thomas P. <thomaspoehlmann96@googlemail.com>")
    .about("Initialize new project")
    .subcommand(App::new("init")
      .about("Initialize new project")
      .arg(Arg::with_name("name")
        .help("name of the project")
        .required(true)
        .index(1))
      .arg(Arg::with_name("template")
        .help("template to use")
        .required(true)
        .index(2))
      .arg(Arg::with_name("dir")
        .help("Project directory")
        .required(true)
        .index(3)))
    .subcommand(App::new("list")
      .about("List all available templates"))
    .get_matches();

    let config = config::init().unwrap();

    match matches.subcommand() {
      ("init", Some(init_matches)) => {
        let name = init_matches.value_of("name").unwrap();
        let template = init_matches.value_of("template").unwrap();
        let dir = init_matches.value_of("dir").unwrap();
        let opts = core::InitOpts {
          name: name.to_string(),
          template: template.to_string(),
          dir: dir.to_string()
        };
        match core::init(&config, opts) {
          Ok(fc) => fc,
          Err(error) => println!("Error occured: {}", error)
        };
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