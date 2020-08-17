use crate::cli::{input, password};
use crate::config::{Config, RepositoryOptions};
use crate::git;
use crate::repository::Repository;

use clap::ArgMatches;

pub fn add(config: &mut Config, args: &ArgMatches) {
    let repository_name = args.value_of("repository");
    //// Get repository name from user input
    let repository_name = if repository_name.is_none() {
        match input("repository name", false) {
            Some(value) => value,
            None => return,
        }
    } else {
        String::from(repository_name.unwrap())
    };
    // validate name
    let repositories = Repository::get_repositories(config);
    if repositories.contains(&repository_name) {
        // TODO error
        return;
    }
    //// Get repository description from user input
    let repository_description = match input("repository description", false) {
        Some(value) => value,
        None => return,
    };
    let mut git_options = git::GitOptions {
        enabled: false,
        url: None,
        auth: None,
        token: None,
        username: None,
        password: None,
    };
    // Enable git
    git_options.enabled = match input("Enable remote repository: (y:n)", false) {
        Some(value) => value == "y",
        None => return,
    };
    // Git options
    if git_options.enabled {
        git_options.url = input("Please enter the remote repository url", false);
        git_options.username = input("Please enter your git username", false);
        git_options.username = match password("Please enter your git password") {
            Ok(value) => Some(value),
            Err(_error) => return,
        }
    }
    config.templates_repositories.push(RepositoryOptions {
        name: repository_name,
        description: repository_description,
        git_options: git_options,
    });
    match config.save() {
        Ok(()) => (),
        Err(_error) => return,
    }
}
