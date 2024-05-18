use anyhow::Result;
use git2::{ErrorCode, Repository};

use self::command::{
    commit_short_sha, commit_tag, local_branch_name, local_repo_changes, merge_base,
    remote_branch_name, remote_name, rev_to_pull, rev_to_push, stash_count,
};

use super::{branch::build_fully_qualified_remote_branch_name, types::GitRepoState};

pub mod command;

pub fn check_in_git_directory() -> Result<bool> {
    match Repository::open_from_env() {
        Ok(_) => Ok(true),
        Err(err) if err.code() == ErrorCode::NotFound => Ok(false),
        Err(err) => Err(err.into()),
    }
}

pub fn get_git_repo_state() -> Result<GitRepoState> {
    let mut repository = Repository::open_from_env()?;
    let local_branch = local_branch_name(&repository)?;
    let git_local_repo_changes = local_repo_changes(&repository)?;
    let remote = remote_name(&repository, &local_branch)?;
    let stash_count = stash_count(&mut repository)?;
    let commit_short_sha = commit_short_sha(&repository)?;
    let commit_tag = commit_tag(&repository)?;

    let mut repo_state = GitRepoState {
        local_branch,
        git_local_repo_changes,
        remote,
        stash_count,
        commit_short_sha,
        commit_tag,
        ..Default::default()
    };

    if !repo_state.remote.is_empty() {
        repo_state.remote_tracking_branch =
            remote_branch_name(&repository, &repo_state.local_branch)?;
        let merge_base = merge_base(&repository, &repo_state.local_branch)?;

        let full_remote_branch_name = build_fully_qualified_remote_branch_name(
            &repo_state.remote,
            &repo_state.remote_tracking_branch,
        );

        repo_state.commits_to_pull = rev_to_pull(&repository, &full_remote_branch_name, "HEAD")?;
        repo_state.commits_to_push = rev_to_push(&repository, &full_remote_branch_name, "HEAD")?;

        if !merge_base.is_empty() {
            repo_state.merge_branch_commits_to_pull =
                rev_to_pull(&repository, "origin/master", &full_remote_branch_name)?;
            repo_state.merge_branch_commits_to_push =
                rev_to_push(&repository, "origin/master", &full_remote_branch_name)?;
        }
    }

    Ok(repo_state)
}
