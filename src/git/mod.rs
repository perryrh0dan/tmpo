use log;
use std::path::Path;
// use crate::error::RunError;

pub mod github;
pub mod gitlab;
pub mod utils;

extern crate git2;
extern crate serde_json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Options {
  pub enabled: bool,
  pub provider: Option<Provider>, // github, gitlab
  pub url: Option<String>,
  pub auth: Option<AuthType>, // basic, none, token
  pub token: Option<String>,
  pub username: Option<String>,
  pub password: Option<String>,
}

#[derive(Deserialize, Serialize, PartialEq, Clone, Debug)]
pub enum Provider {
  #[serde(alias = "github")]
  GITHUB,
  #[serde(alias = "gitlab")]
  GITLAB,
}

#[derive(Deserialize, Serialize, PartialEq, Clone, Debug)]
pub enum AuthType {
  #[serde(alias = "basic")]
  BASIC,
  #[serde(alias = "none")]
  NONE,
  #[serde(alias = "ssh")]
  SSH,
  #[serde(alias = "token")]
  TOKEN,
}

impl Options {
  pub fn new() -> Options {
    Options {
      enabled: false,
      provider: None,
      url: None,
      auth: None,
      token: None,
      username: None,
      password: None,
    }
  }
}

pub fn init(dir: &Path, repository: &str) -> Result<(), git2::Error> {
  // Initialize git repository
  let repo = match git2::Repository::init(dir) {
    Ok(repo) => repo,
    Err(error) => {
      return Err(error);
    }
  };

  // Set remote
  match repo.remote_set_url("origin", repository) {
    Ok(()) => (),
    Err(error) => {
      return Err(error);
    }
  };

  Ok(())
}

pub fn update(dir: &Path, opts: &Options) -> Result<(), git2::Error> {
  let repo = match git2::Repository::open(dir) {
    Ok(repo) => repo,
    Err(e) => return Err(e),
  };

  let remote_name = "origin";
  let remote_branch = "master";
  let mut remote = repo.find_remote(remote_name)?;
  let fetch_commit = do_fetch(&repo, &[remote_branch], &mut remote, opts)?;
  do_merge(&repo, &remote_branch, fetch_commit)
}

fn do_fetch<'a>(
  repo: &'a git2::Repository,
  refs: &[&str],
  remote: &'a mut git2::Remote,
  opts: &Options,
) -> Result<git2::AnnotatedCommit<'a>, git2::Error> {
  // token needs to be declared here to live longer than the fetchOptions

  log::info!("Fetching repository");

  let token;
  let mut fo = git2::FetchOptions::new();
  let mut callbacks = git2::RemoteCallbacks::new();
  let auth = opts.auth.clone().unwrap();

  match auth {
    AuthType::BASIC => {
      log::info!("[git]: authentication using basic");
      if opts.username.is_none() || opts.password.is_none() {
        log::error!("Username or Password is missing");
        return Err(git2::Error::from_str("missing credentials"));
      }
      callbacks.credentials(|_url, _username_from_url, _allowed_types| {
        git2::Cred::userpass_plaintext(
          &opts.username.clone().unwrap(),
          &opts.password.clone().unwrap(),
        )
      });
    }
    AuthType::TOKEN => {
      log::info!("[git]: authentication using token");
      if opts.token.is_none() {
        log::error!("No token was provided");
        return Err(git2::Error::from_str("missing auth token"));
      }
      token = opts.token.clone().unwrap();
      // different behavior for github and gitlab
      callbacks.credentials(|_url, _username_from_url, _allowed_types| {
        match opts.provider.clone().unwrap() {
          Provider::GITHUB => git2::Cred::userpass_plaintext(&token, ""),
          Provider::GITLAB => git2::Cred::userpass_plaintext("oauth2", &token),
        }
      });
    }
    AuthType::SSH => {
      log::info!("[git]: authentication using ssh");
      // username_from_url is only working with an ssh url
      // problems with encrypted private keys
      callbacks.credentials(|url, username_from_url, _allowed_types| {
        if url.contains("git@") {
          git2::Cred::ssh_key(
            username_from_url.unwrap(),
            None,
            std::path::Path::new(&format!(
              "{}/.ssh/id_rsa",
              dirs::home_dir().unwrap().to_string_lossy()
            )),
            // std::path::Path::new(&format!("{}/.ssh/id_rsa", env::var("HOME").unwrap())),
            None,
          )
        } else {
          git2::Cred::ssh_key(
            &opts.username.clone().unwrap(),
            None,
            std::path::Path::new(&format!(
              "{}/.ssh/id_rsa",
              dirs::home_dir().unwrap().to_string_lossy()
            )),
            // std::path::Path::new(&format!("{}/.ssh/id_rsa", env::var("HOME").unwrap())),
            None,
          )
        }
      });
    },
    AuthType::NONE => {},
  }

  // Always fetch all tags.
  // Perform a download and also update tips
  fo.download_tags(git2::AutotagOption::All);
  fo.remote_callbacks(callbacks);

  match remote.fetch(refs, Some(&mut fo), None) {
    Ok(()) => (),
    Err(error) => {
      log::error!("{}", error);
    }
  };
  let fetch_head = repo.find_reference("FETCH_HEAD")?;
  Ok(repo.reference_to_annotated_commit(&fetch_head)?)
}

fn fast_forward(
  repo: &git2::Repository,
  lb: &mut git2::Reference,
  rc: &git2::AnnotatedCommit,
) -> Result<(), git2::Error> {
  let name = match lb.name() {
    Some(s) => s.to_string(),
    None => String::from_utf8_lossy(lb.name_bytes()).to_string(),
  };

  let msg = format!("Fast-Forward: Setting {} to id: {}", name, rc.id());

  lb.set_target(rc.id(), &msg)?;
  repo.set_head(&name)?;
  repo.checkout_head(Some(
    git2::build::CheckoutBuilder::default()
      // For some reason the force is required to make the working directory actually get updated
      // I suspect we should be adding some logic to handle dirty working directory states
      // but this is just an example so maybe not.
      .force(),
  ))?;
  Ok(())
}

fn normal_merge(
  repo: &git2::Repository,
  local: &git2::AnnotatedCommit,
  remote: &git2::AnnotatedCommit,
) -> Result<(), git2::Error> {
  let local_tree = repo.find_commit(local.id())?.tree()?;
  let remote_tree = repo.find_commit(remote.id())?.tree()?;
  let ancestor = repo
    .find_commit(repo.merge_base(local.id(), remote.id())?)?
    .tree()?;
  let mut idx = repo.merge_trees(&ancestor, &local_tree, &remote_tree, None)?;

  if idx.has_conflicts() {
    log::error!("Merge conficts detected...");
    repo.checkout_index(Some(&mut idx), None)?;
    return Ok(());
  }
  let result_tree = repo.find_tree(idx.write_tree_to(repo)?)?;
  // now create the merge commit
  let msg = format!("Merge: {} into {}", remote.id(), local.id());
  let sig = repo.signature()?;
  let local_commit = repo.find_commit(local.id())?;
  let remote_commit = repo.find_commit(remote.id())?;
  // Do our merge commit and set current branch head to that commit.
  let _merge_commit = repo.commit(
    Some("HEAD"),
    &sig,
    &sig,
    &msg,
    &result_tree,
    &[&local_commit, &remote_commit],
  )?;
  // Set working tree to match head.
  repo.checkout_head(None)?;
  Ok(())
}

fn do_merge<'a>(
  repo: &'a git2::Repository,
  remote_branch: &str,
  fetch_commit: git2::AnnotatedCommit<'a>,
) -> Result<(), git2::Error> {
  // 1. do a merge analysis
  let analysis = repo.merge_analysis(&[&fetch_commit])?;

  // 2. Do the appopriate merge
  if analysis.0.is_fast_forward() {
    // do a fast forward
    let refname = format!("refs/heads/{}", remote_branch);
    match repo.find_reference(&refname) {
      Ok(mut r) => {
        fast_forward(repo, &mut r, &fetch_commit)?;
      }
      Err(_) => {
        // The branch doesn't exist so just set the reference to the
        // commit directly. Usually this is because you are pulling
        // into an empty repository.
        repo.reference(
          &refname,
          fetch_commit.id(),
          true,
          &format!("Setting {} to {}", remote_branch, fetch_commit.id()),
        )?;
        repo.set_head(&refname)?;
        repo.checkout_head(Some(
          git2::build::CheckoutBuilder::default()
            .allow_conflicts(true)
            .conflict_style_merge(true)
            .force(),
        ))?;
      }
    };
  } else if analysis.0.is_normal() {
    // do a normal merge
    let head_commit = repo.reference_to_annotated_commit(&repo.head()?)?;
    normal_merge(&repo, &head_commit, &fetch_commit)?;
  }
  Ok(())
}
