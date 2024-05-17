use std::error::Error;

use self::{
    command::{git_cmd_local_branch_name, git_cmd_porcelain_status, git_cmd_remote_name},
    parse::status::git_parse_status,
};

pub mod command;
pub mod parse;
pub mod types;

pub fn get_git_repo_state() -> Result<types::GitRepoState, Box<dyn Error>> {
    let local_branch = git_cmd_local_branch_name()?;
    let git_status = git_cmd_porcelain_status()?;
    let git_local_repo_changes = git_parse_status(&git_status)?;
    let remote = git_cmd_remote_name(&local_branch)?;

    Ok(types::GitRepoState {
        local_branch,
        git_local_repo_changes,
        remote,
        ..Default::default()
    })
}
