use crate::action::Action;
use crate::config;
use crate::out;

impl Action {
    pub fn config(&self) {
        let config_path = config::config_location();

        out::info::display_config(&self.config, &config_path.to_str().unwrap());
    }
}
