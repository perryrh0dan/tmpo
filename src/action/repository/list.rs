use crate::action::Action;
use crate::out;

impl Action {
  pub fn repository_list(self) {
    let repositories = self.config.get_repository_names();

    out::info::list_repositories(&repositories);
  }
}
