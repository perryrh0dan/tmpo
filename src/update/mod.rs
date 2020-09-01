use log;
use std::fs::File;

use crate::out;

extern crate flate2;
extern crate self_update;
extern crate tar;
use clap::crate_version;
use flate2::read::GzDecoder;
use tar::Archive;

pub fn check_version() -> Option<self_update::update::ReleaseAsset> {
    log::info!("Fetch release list");
    let releases = self_update::backends::github::ReleaseList::configure()
        .repo_owner("perryrh0dan")
        .repo_name("tmpo")
        .build()
        .unwrap()
        .fetch()
        .unwrap();

    // check version
    let version = crate_version!();
    if releases[0].version == version {
        log::info!("No update");
        out::info::no_app_update();
        return None;
    }

    println!("Checking target-arch: {}", &self_update::get_target());
    println!("Checking current version: {}", &version);
    println!(
        "New release found! {} --> {}",
        &version, &releases[0].version
    );

    log::info!(
        "New release found! {} --> {}",
        &version,
        &releases[0].version
    );

    let asset = match releases[0].asset_for(&self_update::get_target()) {
        Some(value) => value,
        None => {
            println!("New release is not compatible");
            return None;
        }
    };

    println!("New release is compatible");
    println!();

    return Some(asset);
}

pub fn update(asset: self_update::update::ReleaseAsset) {
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
        Ok(_) => (),
        Err(error) => {
            log::error!("{}", error);
            return;
        }
    };

    // extract tar.gz archive
    log::info!("Extract tar.gz archive");
    let tar_gz = File::open(tmp_tarball_path).unwrap();
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    match archive.unpack(&tmp_dir) {
        Ok(_) => (),
        Err(error) => {
            log::error!("{}", error);
            return;
        }
    };

    // move file to current executable
    #[cfg(not(windows))]
    let bin_name = "tmpo";
    #[cfg(windows)]
    let bin_name = "tmpo.exe";
    let tmp_file = tmp_dir.path().join("replacement_tmp");
    let bin_path = tmp_dir.path().join(bin_name);
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
            log::error!("{}", error);
            match error {
                self_update::errors::Error::Io { .. } => {
                    out::error::selfupdate_no_permission();
                    return;
                }
                _ => {
                    out::error::unknown();
                    return;
                }
            }
        }
    };

    out::success::app_updated();
}
