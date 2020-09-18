use log;
use std::process::exit;

use crate::action::Action;
use crate::cli::input::confirm;
use crate::out;
use crate::update;

impl Action {
  pub fn update(&self) {
    let (_version, asset) = match update::check_version() {
      Some(data) => data,
      None => {
        out::info::no_app_update();
        exit(0)
      }
    };

    // user input
    let update = confirm("The new release will be downloaded/extraced and the existing binary will be replaced.\nDo you want to continue?");
    if !update {
      exit(0);
    }

    println!();

    match update::update(asset) {
      Ok(()) => (),
      Err(error) => {
        log::error!("{}", error);
        eprintln!("{}", error);
        exit(1);
      }
    };

    out::success::app_updated();
  }
}
