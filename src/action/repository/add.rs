use crate::cli::input;
use crate::config::{Config, RepositoryOptions};
use crate::git;
use crate::out;
use crate::utils;

use clap::ArgMatches;

pub fn add(config: &mut Config, args: &ArgMatches) {
    let repository_name = args.value_of("repository");

    //// Get repository name from user input
    let repository_name = if repository_name.is_none() {
      match input::text("repository name", false) {
        Some(value) => value,
        None => return,
      }
    } else {
      utils::lowercase(repository_name.unwrap())
    };

    // validate name
    let repositories = config.get_repositories();
    if repositories.contains(&repository_name) {
        // TODO error
        return;
    }

    // Get repository description from user input
    let repository_description = match input::text("repository description", false) {
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
    git_options.enabled = input::confirm("Enable remote repository?");

    // Git options
    if git_options.enabled {
        // Get repository remote url
        git_options.url = input::text("Please enter the remote repository url", false);

        // Get authentication type
        git_options.auth = match input::select("Auth type", &vec![String::from("basic"), String::from("token"), String::from("none")]) {
            Ok(value) => Some(value),
            Err(_error) => return,
        };

        // Get credentials for different auth types
        if git_options.auth.clone().unwrap() == "basic" {
            git_options.username = input::text("Please enter your git username", false);
            git_options.password = match input::password("Please enter your git password") {
                Ok(value) => Some(value),
                Err(_error) => return,
            }
        } else if git_options.auth.clone().unwrap() == "token" {
            git_options.token = input::text("Please enter your git token", false);
        }
    }

    config.template_repositories.push(RepositoryOptions {
        name: repository_name.to_owned(),
        description: repository_description,
        git_options: git_options,
    });

    match config.save() {
        Ok(()) => (),
        Err(_error) => return,
    }

    out::success::repository_added(&repository_name);
}
