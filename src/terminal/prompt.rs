use crate::{config::types::Config, git::types::GitRepoState};

use super::types::Shell;

pub fn build_prompt(shell: Shell, config: &Config, repo_state: &GitRepoState) -> String {
    let result = String::new();

    result
}
