use crate::config::Config;
use crate::out;

pub fn list(config: &Config) {
    let repositories = config.get_repositories();

    out::info::list_repositories(&repositories);
}
