use anyhow::Result;
use git2::{ErrorCode, Repository, Sort, StatusOptions};

use crate::git::types::GitLocalRepoChanges;

macro_rules! ignore_error_code {
    ($error_code: ident, $stmt: expr, $fallback: expr) => {
        match $stmt {
            Ok(result) => result,
            Err(err) if err.code() == ErrorCode::$error_code => return Ok($fallback.into()),
            Err(err) => return Err(err.into()),
        }
    };
    ($error_code: ident, $stmt: expr) => {
        ignore_error_code!($error_code, $stmt, "")
    };
}

pub fn local_branch_name(repository: &Repository) -> Result<String> {
    let head_ref = repository.find_reference("HEAD")?;

    if let Some(symbolic_target) = head_ref.symbolic_target() {
        if let Some(branch_name) = symbolic_target.strip_prefix("refs/heads/") {
            return Ok(branch_name.into());
        }
    }
    Ok("".into())
}

pub fn local_repo_changes(repository: &Repository) -> Result<GitLocalRepoChanges> {
    let statuses = repository.statuses(Some(
        StatusOptions::new()
            .include_untracked(true)
            .renames_head_to_index(true),
    ))?;
    let mut result = GitLocalRepoChanges::default();

    for entry in statuses.into_iter() {
        let status = entry.status();

        if status.is_conflicted() {
            result.conflict += 1;
        } else if status.is_index_renamed() {
            result.renamed += 1;
        } else if status.is_index_new() {
            result.index_add += 1;
        } else if status.is_index_deleted() {
            result.index_del += 1;
        } else if status.is_index_modified() {
            result.index_mod += 1;
        }

        if status.is_wt_new() {
            result.local_add += 1;
        } else if status.is_wt_deleted() {
            result.local_del += 1;
        } else if status.is_wt_modified() {
            result.local_mod += 1;
        }
    }

    Ok(result)
}

pub fn remote_default_branch(repository: &Repository, remote: &str) -> Result<String> {
    let remote_head = ignore_error_code!(
        NotFound,
        repository.find_reference(&format!("refs/remotes/{remote}/HEAD")),
        format!("{remote}/master")
    );
    if let Some(remote_ref) = remote_head.symbolic_target() {
        if let Some(remote_branch) = remote_ref.strip_prefix("refs/remotes/") {
            return Ok(remote_branch.into());
        }
    }
    Ok(format!("{remote}/master"))
}

pub fn merge_base(
    repository: &Repository,
    remote_default_branch: &str,
    local_branch_name: &str,
) -> Result<String> {
    let remote_oid = ignore_error_code!(
        NotFound,
        repository.refname_to_id(&format!("refs/remotes/{remote_default_branch}"))
    );
    let local_oid = repository.refname_to_id(&format!("refs/heads/{local_branch_name}"))?;

    Ok(repository.merge_base(remote_oid, local_oid)?.to_string())
}

pub fn remote_name(repository: &Repository, local_branch_name: &str) -> Result<String> {
    let result = ignore_error_code!(
        NotFound,
        repository
            .config()?
            .get_string(&git_remote_tracking_config_key(local_branch_name))
    );

    Ok(result)
}

pub fn remote_branch_name(repository: &Repository, local_branch_name: &str) -> Result<String> {
    let result = ignore_error_code!(
        NotFound,
        repository
            .config()?
            .get_string(&git_remote_branch_config_key(local_branch_name))
    );

    Ok(result)
}

pub fn stash_count(repository: &mut Repository) -> Result<usize> {
    let mut count = 0;
    repository.stash_foreach(|_, _, _| {
        count += 1;
        true
    })?;

    Ok(count)
}

pub fn commit_short_sha(repository: &Repository) -> Result<String> {
    let head = ignore_error_code!(UnbornBranch, repository.head());
    let head_commit = head.peel_to_commit()?;
    let oid = head_commit.id();

    Ok((&oid.to_string()[..7]).into())
}

pub fn commit_tag(repository: &Repository) -> Result<String> {
    let head = ignore_error_code!(UnbornBranch, repository.head());
    let head_commit = head.peel_to_commit()?;
    let oid = head_commit.id();

    let tags = repository.tag_names(None)?;

    for tag_name in tags.into_iter().flatten() {
        let tag_ref = repository.find_reference(&format!("refs/tags/{tag_name}"))?;
        if let Some(tag_oid) = tag_ref.target() {
            if tag_oid == oid {
                return Ok(tag_name.into());
            }
        }
    }
    Ok("".into())
}

pub fn rev_to_push(repository: &Repository, from_commit: &str, to_commit: &str) -> Result<usize> {
    let from_oid = repository.revparse_single(from_commit)?.id();
    let to_oid = repository.revparse_single(to_commit)?.id();

    let mut revwalk = repository.revwalk()?;
    revwalk.set_sorting(Sort::TOPOLOGICAL)?;
    revwalk.push(to_oid)?;
    revwalk.hide(from_oid)?;

    let mut count = 0;

    for oid in revwalk {
        let commit = repository.find_commit(oid?)?;

        if commit.parent_count() == 1 {
            count += 1;
        }
    }

    Ok(count)
}

pub fn rev_to_pull(repository: &Repository, from_commit: &str, to_commit: &str) -> Result<usize> {
    let from_oid = repository.revparse_single(from_commit)?.id();
    let to_oid = repository.revparse_single(to_commit)?.id();

    let mut revwalk = repository.revwalk()?;
    revwalk.set_sorting(Sort::TOPOLOGICAL)?;
    revwalk.push(from_oid)?;
    revwalk.hide(to_oid)?;

    let mut count = 0;

    for oid in revwalk {
        let commit = repository.find_commit(oid?)?;

        if commit.parent_count() == 1 {
            count += 1;
        }
    }

    Ok(count)
}

fn git_remote_tracking_config_key(local_branch_name: &str) -> String {
    format!("branch.{local_branch_name}.remote")
}

fn git_remote_branch_config_key(local_branch_name: &str) -> String {
    format!("branch.{local_branch_name}.merge")
}
