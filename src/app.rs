use crate::crate_version;

use clap::{arg, ArgAction, Command};

pub fn build() -> Command {
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
                .required(false),
        )
        .arg(arg!(--remote <URL> "Remote URL").required(false))
        .arg(arg!(-r --repository <NAME> "Repository to use").required(false))
        .arg(arg!(-t --template <NAME> "Template to use for generation").required(false))
        .arg(arg!(--username <USERNAME> "Username of the user").required(false))
        .arg(arg!(--email <EMAIL> "E-Mail of the user").required(false))
        .arg(
            arg!(no_script: --"no-script" "Don't execute template scripts")
                .action(ArgAction::SetTrue)
                .required(false),
        );

    let repository_subcommand = Command::new("repository")
        .about("Maintain repositories")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("add")
                .about("Add repository")
                .arg(arg!(-t --type <TYPE> "Type of the repository").required(false))
                .arg(arg!(-n --name <NAME> "Name of the repository").required(false))
                .arg(
                    arg!(-d --description <DESCRIPTION> "Description of the repository")
                        .required(false),
                )
                .arg(arg!(--provider <PROVIDER> "Remote provider").required(false))
                .arg(arg!(--authentication <TYPE> "Authentication type").required(false))
                .arg(arg!(--url <URL> "Remote url of the repository").required(false))
                .arg(arg!(--branch <BRANCH> "Remote repository branch").required(false))
                .arg(arg!(--username <USERNAME> "Username for authentication").required(false))
                .arg(
                    arg!(--password <PASSWORD> "Password for basic authentication").required(false),
                )
                .arg(arg!(--token <TOKEN> "Token for authentication").required(false)),
        )
        .subcommand(
            Command::new("create")
                .about("Create a new repository")
                .arg(arg!(-t --type <TYPE> "Type of the repository").required(false))
                .arg(arg!(-n --name <NAME> "Name of the repository").required(false))
                .arg(
                    arg!(-d --description <DESCRIPTION> "Description of the repository")
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
                .arg(arg!(-r --repository <NAME> "Name of the repository").required(false)),
        )
        .subcommand(
            Command::new("view")
                .about("View repository details")
                .arg(arg!(-r --repository <NAME> "Name of the repository").required(false)),
        );

    let template_subcommand = Command::new("template")
        .about("Maintain templates")
        .arg_required_else_help(true)
        .subcommand(
            Command::new("add")
                .about("Add a single template repository")
                .arg(arg!(--url <URL> "Remote url of the template").required(false)),
        )
        .subcommand(
            Command::new("create")
                .about("Create new template")
                .arg(arg!(-r --repository <NAME> "Name of the repository").required(false))
                .arg(arg!(-n --name <NAME> "Name of the template").required(false)),
        )
        .subcommand(
            Command::new("list")
                .about("List all available templates")
                .alias("ls")
                .arg(arg!(-r --repository <NAME> "Name of the repository").required(false)),
        )
        .subcommand(
            Command::new("remove")
                .about("Remove a template")
                .alias("rm")
                .arg(arg!(-t --template <NAME> "Template name").required(false)),
        )
        .subcommand(
            Command::new("test")
                .about("Test template at a given location")
                .arg(arg!(-d --directory <PATH> "Directory of the template").required(true)),
        )
        .subcommand(
            Command::new("view")
                .about("View template details")
                .arg(arg!(-r --repository <NAME> "Name of the repository").required(false))
                .arg(arg!(-t --template <NAME> "Name of the template").required(false)),
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
                .action(ArgAction::SetTrue)
                .required(false)
                .global(true),
        )
        .arg(
            arg!(yes: -y --yes "Skips all optional questions")
                .action(ArgAction::SetTrue)
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
