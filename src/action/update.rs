extern crate self_update;
use self_update::cargo_crate_version;

pub fn update() {
  let update = self_update::backends::github::Update::configure()
    .repo_owner("perryrh0dan")
    .repo_name("charon")
    .bin_name("charon")
    .show_download_progress(true)
    .current_version(cargo_crate_version!())
    .build();

  if !update.is_ok() {
    return;
  };

  let status = update.unwrap().update();

  if !status.is_ok() {
    return;
  };

  println!("Update status: `{}`!", status.unwrap().version());
}
