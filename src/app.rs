use crate::crate_version;

use clap::{arg, Command};

pub fn build() -> Command<'static> {
  let init_subcommand = Command::new("init")
    .about("Initialize new workspace")
    .visible_alias("i")
    .arg(
      arg!([name] "Name of the new workspace and the project.")
        .required(false)
        .index(1),
    )
    .arg(
      arg!(-d --directory <PATH> "Directory name to create the workspace in.")
        .takes_value(true)
        .required(false),
    )
    .arg(
      arg!(--remote <URL> "Remote URL")
        .takes_value(true)
        .required(false),
    )
    .arg(
      arg!(-r --repository <NAME> "Repository to use")
        .takes_value(true)
        .required(false),
    )
    .arg(
      arg!(-t --template <NAME> "Template to use for generation")
        .takes_value(true)
        .required(false),
    )
    .arg(
      arg!(--username <USERNAME> "Username of the user")
        .takes_value(true)
        .required(false),
    )
    .arg(
      arg!(--email <EMAIL> "E-Mail of the user")
        .takes_value(true)
        .required(false),
    )
    .arg(
      arg!(no_script: --"no-script" "Don't execute template scripts")
        .takes_value(false)
        .required(false),
    );

  let repository_subcommand = Command::new("repository")
    .about("Maintain repositories")
    .subcommand_required(true)
    .arg_required_else_help(true)
    .subcommand(
      Command::new("add")
        .about("Add repository")
        .arg(
          arg!(-t --type <TYPE> "Type of the repository")
            .takes_value(true)
            .required(false),
        )
        .arg(
          arg!(-n --name <NAME> "Name of the repository")
            .takes_value(true)
            .required(false),
        )
        .arg(
          arg!(-d --description <DESCRIPTION> "Description of the repository")
            .takes_value(true)
            .required(false),
        )
        .arg(
          arg!(--provider <PROVIDER> "Remote provider")
            .takes_value(true)
            .required(false),
        )
        .arg(
          arg!(--authentication <TYPE> "Authentication type")
            .takes_value(true)
            .required(false),
        )
        .arg(
          arg!(--url <URL> "Remote url of the repository")
            .takes_value(true)
            .required(false),
        )
        .arg(
          arg!(--branch <BRANCH> "Remote repository branch")
            .takes_value(true)
            .required(false),
        )
        .arg(
          arg!(--username <USERNAME> "Username for authentication")
            .takes_value(true)
            .required(false),
        )
        .arg(
          arg!(--password "Password for basic authentication")
            .takes_value(true)
            .required(false),
        )
        .arg(
          arg!(--token "Token for authentication")
            .takes_value(true)
            .required(false),
        ),
    )
    .subcommand(
      Command::new("create")
        .about("Create a new repository")
        .arg(
          arg!(-t --type <TYPE> "Type of the repository")
            .takes_value(true)
            .required(false),
        )
        .arg(
          arg!(-n --name <NAME> "Name of the repository")
            .takes_value(true)
            .required(false),
        )
        .arg(
          arg!(-d --description <DESCRIPTION> "Description of the repository")
            .takes_value(true)
            .required(false),
        ),
    )
    .subcommand(
      Command::new("list")
        .about("List all available repository")
        .alias("ls"),
    )
    .subcommand(
      Command::new("remove")
        .about("Remove a repository")
        .alias("rm")
        .arg(
          arg!(-r --repository <NAME> "Name of the repository")
            .takes_value(true)
            .required(false),
        ),
    )
    .subcommand(
      Command::new("view").about("View repository details").arg(
        arg!(-r --repository <NAME> "Name of the repository")
          .takes_value(true)
          .required(false),
      ),
    );

  let template_subcommand = Command::new("template")
    .about("Maintain templates")
    .arg_required_else_help(true)
    .subcommand(
      Command::new("add")
        .about("Add a single template repository")
        .arg(
          arg!(--url <URL> "Remote url of the template")
            .takes_value(true)
            .required(false),
        ),
    )
    .subcommand(
      Command::new("create")
        .about("Create new template")
        .arg(
          arg!(-r --repository <NAME> "Name of the repository")
            .takes_value(true)
            .required(false),
        )
        .arg(
          arg!(-n --name <NAME> "Name of the template")
            .takes_value(true)
            .required(false),
        ),
    )
    .subcommand(
      Command::new("list")
        .about("List all available templates")
        .alias("ls")
        .arg(
          arg!(-r --repository <NAME> "Name of the repository")
            .takes_value(true)
            .required(false),
        ),
    )
    .subcommand(
      Command::new("remove")
        .about("Remove a template")
        .alias("rm")
        .arg(
          arg!(-t --template <NAME> "Template name")
            .takes_value(true)
            .required(false),
        ),
    )
    .subcommand(
      Command::new("test")
        .about("Test template at a given location")
        .arg(
          arg!(-d --directory <PATH> "Directory of the template")
            .takes_value(true)
            .required(true),
        ),
    )
    .subcommand(
      Command::new("view")
        .about("View template details")
        .arg(
          arg!(-r --repository <NAME> "Name of the repository")
            .takes_value(true)
            .required(false),
        )
        .arg(
          arg!(-t --template <NAME> "Name of the template")
            .takes_value(true)
            .required(false),
        ),
    );

  let app = Command::new("tmpo")
    .version(crate_version!())
    .propagate_version(true)
    .author("Thomas P. <thomaspoehlmann96@googlemail.com>")
    .about("Cli to create new workspaces based on templates")
    .subcommand_required(true)
    .arg_required_else_help(true)
    .help_expected(true)
    .arg(
      arg!(verbose: -v --verbose "Adds more details to output logging")
        .takes_value(false)
        .required(false)
        .global(true),
    )
    .arg(
      arg!(yes: -y --yes "Skips all optional questions")
        .takes_value(false)
        .required(false)
        .global(true),
    )
    .subcommand(init_subcommand)
    .subcommand(Command::new("config").about("View configuration"))
    .subcommand(Command::new("update").about("Update to the latest release"))
    .subcommand(repository_subcommand)
    .subcommand(template_subcommand);

  return app;
}

#[test]
fn verify_app() {
  build().debug_assert()
}
