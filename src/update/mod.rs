use log;
use std::fs::File;

use crate::error::RunError;


use crate::crate_version;
extern crate flate2;
extern crate self_update;
extern crate tar;
extern crate semver;
use flate2::read::GzDecoder;
use tar::Archive;
use semver::Version;

#[cfg(windows)]
const BIN_NAME: &str = "tmpo.exe";

#[cfg(not(windows))]
const BIN_NAME: &str = "tmpo";

pub fn check_version() -> Option<(Version, self_update::update::ReleaseAsset)> {
  log::info!("Fetch release list");
  let releases = match self_update::backends::github::ReleaseList::configure()
    .repo_owner("perryrh0dan")
    .repo_name("tmpo")
    .build()
    .unwrap()
    .fetch()
  {
    Ok(releases) => releases,
    Err(error) => {
      log::error!("{}", error);
      return None;
    }
  };

  // Check version
  // TODO better check
  let version = crate_version!();
  if releases[0].version == version {
    return None;
  }

  let mut target = self_update::get_target();

  // Needs to be investigated
  if target == "x86_64-pc-windows-msvc" {
    target = "x86_64-pc-windows-gnu";
  }

  let asset = match releases[0].asset_for(target, None) {
    Some(value) => value,
    None => {
      log::info!("New release is not compatible");
      return None;
    }
  };

  log::info!(
    "New release found! {} --> {}",
    &version,
    &releases[0].version
  );

  let target_version = Version::parse(&releases[0].version).unwrap();

  return Some((target_version, asset));
}

pub fn update(asset: self_update::update::ReleaseAsset) -> Result<(), RunError> {
  let tmp_dir = tempfile::Builder::new()
    .tempdir_in(::std::env::current_dir().unwrap())
    .unwrap();
  let tmp_tarball_path = tmp_dir.path().join(&asset.name);
  std::fs::File::create(&tmp_tarball_path).unwrap();
  let tmp_tarball = std::fs::OpenOptions::new()
    .create(true)
    .append(true)
    .open(&tmp_tarball_path)
    .unwrap();

  // download asset
  let mut headers = reqwest::header::HeaderMap::new();
  headers.insert(
    reqwest::header::ACCEPT,
    "application/octet-stream".parse().unwrap(),
  );

  log::info!(
    "Download asset to: {}",
    &tmp_tarball_path.to_owned().to_str().unwrap()
  );
  match self_update::Download::from_url(&asset.download_url)
    .show_progress(true)
    .set_headers(headers)
    .download_to(&tmp_tarball)
  {
    Ok(()) => (),
    Err(_error) => return Err(RunError::Update(String::from("Unknwon"))),
  };

  // extract tar.gz archive
  log::info!("Extract tar.gz archive");
  let tar_gz = File::open(tmp_tarball_path).unwrap();
  let tar = GzDecoder::new(tar_gz);
  let mut archive = Archive::new(tar);
  match archive.unpack(&tmp_dir) {
    Ok(_) => (),
    Err(_error) => return Err(RunError::Update(String::from("Unknwon"))),
  };

  // move file to current executable
  let tmp_file = tmp_dir.path().join("replacement_tmp");
  let bin_path = tmp_dir.path().join(BIN_NAME);
  let dest_path = std::env::current_exe().unwrap();

  log::info!(
    "Move {} to {}",
    bin_path.to_owned().to_str().unwrap(),
    dest_path.to_owned().to_str().unwrap()
  );
  match self_update::Move::from_source(&bin_path)
    .replace_using_temp(&tmp_file)
    .to_dest(&dest_path)
  {
    Ok(_) => (),
    Err(error) => {
      return match error {
        self_update::errors::Error::Io { .. } => {
          Err(RunError::Update(String::from("No permission")))
        }
        _ => Err(RunError::Update(String::from("Unknown"))),
      }
    }
  };

  Ok(())
}
