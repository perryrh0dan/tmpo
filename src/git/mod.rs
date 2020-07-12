use std::str;
use std::path::Path;

use crate::renderer;

#[macro_use]
use log::debug;

extern crate custom_error;
extern crate git2;
use custom_error::custom_error;

custom_error! {pub GitError
    InitError      = "Unable to initialize git",
    AddRemoteError = "Unable to add remote",
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct GitOptions {
  pub enabled: bool,
  pub url: Option<String>,
  pub auth: Option<String>,
  pub token: Option<String>,
  pub username: Option<String>,
  pub password: Option<String>,
}

pub fn init(dir: &Path, repository: &str) -> Result<(), GitError> {
  // Initialize git repository
  let repo = match git2::Repository::init(dir) {
    Ok(repo) => repo,
    Err(_e) => return Err(GitError::InitError),
  };

  // Set remote
  match repo.remote_set_url("origin", repository) {
    Ok(()) => (),
    Err(_e) => return Err(GitError::AddRemoteError),
  }

  Ok(())
}

pub fn update(dir: &Path, opts: &GitOptions) -> Result<(), git2::Error> {
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

pub fn get_email() -> Result<String, git2::Error> {
  let config = get_config()?;
  let email = config.get_string("user.email")?;

  let mut buf = String::with_capacity(email.len());

  for c in email.chars() {
    buf.push(c);
  }

  Ok(buf)
}

pub fn get_username() -> Result<String, git2::Error> {
  let config = get_config()?;
  let username = config.get_string("user.name")?;

  let mut buf = String::with_capacity(username.len());

  for c in username.chars() {
    buf.push(c);
  }

  Ok(buf)
}

// load global git config
fn get_config() -> Result<git2::Config, git2::Error> {
  let path = git2::Config::find_global()?;
  let config = git2::Config::open(&path)?;

  // for entry in &config.entries(None).unwrap() {
  //     let entry = entry.unwrap();
  //     println!("{} => {}", entry.name().unwrap(), entry.value().unwrap());
  // }
  Ok(config)
}

fn do_fetch<'a>(
  repo: &'a git2::Repository,
  refs: &[&str],
  remote: &'a mut git2::Remote,
  opts: &GitOptions,
) -> Result<git2::AnnotatedCommit<'a>, git2::Error> {
  // token needs to be declared here to live longer than the fetchOptions
  let token;
  let mut fo = git2::FetchOptions::new();
  let mut callbacks = git2::RemoteCallbacks::new();
  let auth = opts.auth.clone().unwrap();

  if auth == "ssh" {
    log::debug!("[git]: authentication using ssh");
  // callbacks.credentials(|_url, username_from_url, _allowed_types| {
  //     git2::Cred::ssh_key(
  //     username_from_url.unwrap(),
  //     None,
  //     std::path::Path::new(&format!("{}/.ssh/id_rsa", env::var("HOME").unwrap())),
  //     None,
  //     )
  // });
  } else if auth == "token" {
    debug!("[git]: authentication using token");
    if opts.token.is_none() {
      renderer::errors::missing_token();
      return Err(git2::Error::from_str("missing auth token"));
    }
    token = opts.token.clone().unwrap();
    callbacks.credentials(|_url, _username_from_url, _allowed_types| {
      git2::Cred::userpass_plaintext(&token, "")
    });
  } else {
    debug!("[git]: no authentication");
  }

  // Always fetch all tags.
  // Perform a download and also update tips
  fo.download_tags(git2::AutotagOption::All);
  fo.remote_callbacks(callbacks);

  match remote.fetch(refs, Some(&mut fo), None) {
    Ok(()) => (),
    Err(error) => println!("{}", error.message()),
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
    println!("Merge conficts detected...");
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
        renderer::success_update_templates()
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
