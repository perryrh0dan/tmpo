use std::fs::File;

use crate::out;

extern crate tar;
extern crate self_update;
extern crate flate2;
use clap::{crate_version};
use tar::Archive;
use flate2::read::GzDecoder;


pub fn update() -> Result<(), Box<::std::error::Error>> {
  let releases = self_update::backends::github::ReleaseList::configure()
    .repo_owner("perryrh0dan")
    .repo_name("tmpo")
    .build()?
    .fetch()?;

  // check version 
  let version = crate_version!();
  if releases[0].version == version {
    out::no_app_update();
    return Ok(());
  }

  let asset = releases[0].asset_for(&self_update::get_target()).unwrap();

  let tmp_dir = tempfile::Builder::new().tempdir_in(::std::env::current_dir()?)?;
  let tmp_tarball_path = tmp_dir.path().join(&asset.name);
  std::fs::File::create(&tmp_tarball_path)?;
  let tmp_tarball = std::fs::OpenOptions::new().create(true).append(true).open(&tmp_tarball_path).unwrap();

  // download asset
  let mut headers = reqwest::header::HeaderMap::new();
  headers.insert(reqwest::header::ACCEPT, "application/octet-stream".parse().unwrap());
  match self_update::Download::from_url(&asset.download_url).show_progress(true).set_headers(headers).download_to(&tmp_tarball) {
    Ok(_) => (),
    Err(error) => println!("{}", error)
  };

  // extract tar.gz archive
  let tar_gz = File::open(tmp_tarball_path).unwrap();
  let tar = GzDecoder::new(tar_gz);
  let mut archive = Archive::new(tar);
  match archive.unpack(&tmp_dir) {
    Ok(_) => (),
    Err(error) => println!("{}", error)
  };

  // move file to current executable
  let bin_name = "tmpo";
  let tmp_file = tmp_dir.path().join("replacement_tmp");
  let bin_path = tmp_dir.path().join(bin_name);
  self_update::Move::from_source(&bin_path)
    .replace_using_temp(&tmp_file)
    .to_dest(&::std::env::current_exe()?)?;

  // remove tmp folder
  match std::fs::remove_dir_all(tmp_dir) {
    Ok(_) => (),
    Err(error) => println!("{}", error),
  };

  out::success_update_app();
  Ok(())
}
