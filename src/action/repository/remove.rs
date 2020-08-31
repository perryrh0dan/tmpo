use std::io::ErrorKind;

use crate::cli::input::select;
use crate::config::{Config};
use crate::out;
use crate::repository::Repository;

use clap::ArgMatches;

pub fn remove(config: &mut Config, args: &ArgMatches) {
    let repository_name = args.value_of("repository");
    //// Get repository name from user input
    let repository_name = if repository_name.is_none() {
        let repositories = config.get_repositories();
        match select("repository", &repositories) {
            Ok(value) => value,
            Err(error) => match error.kind() {
                ErrorKind::InvalidData => {
                    out::errors::no_repositories();
                    return;
                }
                _ => return,
            },
        }
    } else {
        String::from(repository_name.unwrap())
    };
    // remove template folder
    match Repository::delete_repository(config, &repository_name) {
        Ok(()) => (),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => (),
            _ => return,
        },
    };
    // Update config
    let index = config
        .templates_repositories
        .iter()
        .position(|x| x.name == repository_name)
        .unwrap();
    config.templates_repositories.remove(index);
    match config.save() {
        Ok(()) => (),
        Err(_error) => return,
    }
}
