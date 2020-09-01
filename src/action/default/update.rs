use crate::cli::input::confirm;
use crate::update;

pub fn update() {
  let asset = match update::check_version() {
    Some(asset) => asset,
    None => return,
  };

  // user input
  
  let update = confirm("The new release will be downloaded/extraced and the existing binary will be replaced.\nDo you want to continue?");
  if !update {
      return;
  }

  println!();

  update::update(asset);
}
