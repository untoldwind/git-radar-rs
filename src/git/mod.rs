use anyhow::Result;

use self::{
    cli::command::{
        git_cmd_commit_short_sha, git_cmd_commit_tag, git_cmd_local_branch_name,
        git_cmd_merge_base, git_cmd_porcelain_status, git_cmd_remote_branch_name,
        git_cmd_remote_name, git_cmd_rev_to_pull, git_cmd_rev_to_push, git_cmd_stash_count,
    },
    cli::{branch::build_fully_qualified_remote_branch_name, status::git_parse_status},
};

pub mod cli;
pub mod types;

pub fn get_git_repo_state() -> Result<types::GitRepoState> {
    let local_branch = git_cmd_local_branch_name()?;
    let git_status = git_cmd_porcelain_status()?;
    let git_local_repo_changes = git_parse_status(&git_status)?;
    let remote = git_cmd_remote_name(&local_branch)?;
    let stash_count = git_cmd_stash_count()?;
    let commit_short_sha = git_cmd_commit_short_sha()?;
    let commit_tag = git_cmd_commit_tag()?;

    let mut repo_state = types::GitRepoState {
        local_branch,
        git_local_repo_changes,
        remote,
        stash_count,
        commit_short_sha,
        commit_tag,
        ..Default::default()
    };

    if !repo_state.remote.is_empty() {
        repo_state.remote_tracking_branch = git_cmd_remote_branch_name(&repo_state.local_branch)?;
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
