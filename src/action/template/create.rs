// ask for private or public template
// if public try to push
use log;
use std::io::ErrorKind;

use crate::cli::input;
use crate::config::{Config};
use crate::out;
use crate::repository::{Repository, RepositoryError};

use clap::ArgMatches;

pub fn create(config: &mut Config, args: &ArgMatches) {
    let repository_name = args.value_of("repository");
    let template_name = args.value_of("template");

      //// Get repository name from user input
    let repository_name = if repository_name.is_none() {
        let repositories = config.get_repositories();
        match input::select("repository", &repositories) {
        Ok(value) => value,
        Err(error) => match error.kind() {
            ErrorKind::InvalidData => {
            out::errors::no_repositories();
            return;
            },
            _ => std::process::exit(130),
        },
        }
    } else {
        String::from(repository_name.unwrap())
    };

    // Load repository
    let repository = match Repository::new(config, &repository_name) {
        Ok(repository) => repository,
        Err(error) => match error {
        RepositoryError::NotFound => return out::errors::repository_not_found(&repository_name),
        _ => return,
        },
    };

    //// Get template name from user input
    let template_name = if template_name.is_none() {
        match input::text("template name", false) {
            Some(value) => value,
            None => return,
        }
    } else {
        String::from(template_name.unwrap())
    };

    // validate name
    let templates = repository.get_templates();
    if templates.contains(&template_name) {
        // TODO error
        return;
    }

    match repository.create_template(config, &template_name) {
        Ok(()) => (),
        Err(error) => {
            log::error!("{}", error);
        }
    };
}
