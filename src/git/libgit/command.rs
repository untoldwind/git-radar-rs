use anyhow::Result;
use git2::{ErrorCode, Repository, StatusOptions};

use crate::git::types::GitLocalRepoChanges;

pub fn local_branch_name(repository: &Repository) -> Result<String> {
    let head = repository.head()?;

    if let Some(shorthand) = head.shorthand() {
        Ok(shorthand.into())
    } else {
        Ok("".into())
    }
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

pub fn remote_name(repository: &Repository, local_branch_name: &str) -> Result<String> {
    match repository.config()?.get_string(&git_remote_tracking_config_key(local_branch_name)) {
        Ok(str) => Ok(str),
        Err(err) if err.code() == ErrorCode::NotFound => Ok("".into()),
        Err(err) => Err(err.into()),
    }
}

pub fn remote_branch_name(repository: &Repository, local_branch_name: &str) -> Result<String> {
    match repository.config()?.get_string(&git_remote_branch_config_key(local_branch_name)) {
        Ok(str) => Ok(str),
        Err(err) if err.code() == ErrorCode::NotFound => Ok("".into()),
        Err(err) => Err(err.into()),
    }
}

pub fn stash_count(repository: &mut Repository) -> Result<usize> {
    let mut count = 0;
    repository.stash_foreach(|_, _, _| {
        count += 1;
        true
    })?;

    Ok(count)
}


fn git_remote_tracking_config_key(local_branch_name: &str) -> String {
    format!("branch.{}.remote", local_branch_name)
}

fn git_remote_branch_config_key(local_branch_name: &str) -> String {
    format!("branch.{}.merge", local_branch_name)
}
