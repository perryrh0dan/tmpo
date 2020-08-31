use crate::config;
use crate::out;

pub fn config(config: &config::Config) {
    let config_path = config::config_location();

    out::info::display_config(config, &config_path.to_str().unwrap());
}