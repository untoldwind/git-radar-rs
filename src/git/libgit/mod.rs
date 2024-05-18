use anyhow::Result;
use git2::{ErrorCode, Repository};

use self::command::{local_branch_name, local_repo_changes, remote_branch_name, remote_name, stash_count};

use super::{
    cli::{
        branch::build_fully_qualified_remote_branch_name,
        command::{
            git_cmd_commit_short_sha, git_cmd_commit_tag, git_cmd_merge_base,
            git_cmd_rev_to_pull,
            git_cmd_rev_to_push,
        },
    },
    types::GitRepoState,
};

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
    let commit_short_sha = git_cmd_commit_short_sha()?;
    let commit_tag = git_cmd_commit_tag()?;

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
        repo_state.remote_tracking_branch = remote_branch_name(&repository, &repo_state.local_branch)?;
        let merge_base = git_cmd_merge_base(&repo_state.local_branch)?;

        let full_remote_branch_name = build_fully_qualified_remote_branch_name(
            &repo_state.remote,
            &repo_state.remote_tracking_branch,
        );

        repo_state.commits_to_pull = git_cmd_rev_to_pull(&full_remote_branch_name, "HEAD")?;
        repo_state.commits_to_push = git_cmd_rev_to_push(&full_remote_branch_name, "HEAD")?;

        if !merge_base.is_empty() {
            repo_state.merge_branch_commits_to_pull =
                git_cmd_rev_to_pull("origin/master", &full_remote_branch_name)?;
            repo_state.merge_branch_commits_to_push =
                git_cmd_rev_to_push("origin/master", &full_remote_branch_name)?;
        }
    }

    Ok(repo_state)
}
