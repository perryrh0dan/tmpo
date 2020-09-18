use crate::action::Action;
use crate::out;

impl Action {
    pub fn repository_list(self) {
        let repositories = self.config.get_repositories();

        out::info::list_repositories(&repositories);
    }
}
