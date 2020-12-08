mod two;

use semver::Version;

use crate::config;
use crate::error::RunError;

use clap::crate_version;

pub fn check() -> Result<(), RunError> {
  let config_version = config::version()?;
  let application_version = Version::parse(crate_version!()).unwrap();

  if config_version != application_version {
    migrate(config_version, application_version);
  }

  Ok(())
}

fn migrate(current_version: Version, target_version: Version) {
  if current_version < Version::parse("2.0.0").unwrap() && target_version >= Version::parse("2.0.0").unwrap() {
    two::migrate()
  }
}
