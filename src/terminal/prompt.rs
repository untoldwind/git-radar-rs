use crate::{config::types::Config, git::types::GitRepoState};
use std::fmt::{self, Write};

use super::{
    output::TerminalOutput,
    types::{ColoredTag, Shell},
};

pub struct Prompt {
    shell: Shell,
    config: Config,
    repo_state: GitRepoState,
}

impl Prompt {
    pub fn new(shell: Shell, config: Config, repo_state: GitRepoState) -> Prompt {
        Self {
            shell,
            config,
            repo_state,
        }
    }

    fn add_repo_indicator<W: Write>(&self, output: &mut TerminalOutput<W>) -> fmt::Result {
        output.write_str(&self.config.repo_indicator)?;
        output.add_delimter();
        Ok(())
    }

    fn add_no_tracked_upstream_indicator<W: Write>(
        &self,
        output: &mut TerminalOutput<W>,
    ) -> fmt::Result {
        if self.repo_state.remote_tracking_branch.is_empty() {
            output.colored_tag(&self.config.no_tracked_upstream_string)?;
            output.add_delimter();
            output.colored_tag(&self.config.no_tracked_upstream_indicator)?;
            output.add_delimter();
        }
        Ok(())
    }

    fn add_merge_branch_commits<W: Write>(&self, output: &mut TerminalOutput<W>) -> fmt::Result {
        let push = self.repo_state.merge_branch_commits_to_push;
        let pull = self.repo_state.merge_branch_commits_to_pull;

        if push > 0 && pull > 0 {
            output.write_str(&self.config.merge_branch_commits_indicator)?;
            output.add_delimter();
            write!(output, "{pull}")?;
            output.colored_tag(&self.config.merge_branch_commits_both_pull_push)?;
            output.add_delimter();
            write!(output, "{push}")?;
            output.add_delimter();
        } else if pull > 0 {
            output.write_str(&self.config.merge_branch_commits_indicator)?;
            output.add_delimter();
            output.colored_tag(&self.config.merge_branch_commits_only_pull)?;
            output.add_delimter();
            write!(output, "{pull}")?;
            output.add_delimter();
        } else if push > 0 {
            output.write_str(&self.config.merge_branch_commits_indicator)?;
            output.add_delimter();
            output.colored_tag(&self.config.merge_branch_commits_only_push)?;
            output.add_delimter();
            write!(output, "{push}")?;
            output.add_delimter();
        }
        Ok(())
    }

    fn add_local_branch_name<W: Write>(&self, output: &mut TerminalOutput<W>) -> fmt::Result {
        output.write_str(&self.config.local_branch_name_prefix)?;

        if !self.repo_state.local_branch.is_empty() {
            output.string_in_color(
                self.config.local_branch_color,
                &self.repo_state.local_branch,
            )?;
        } else if !self.repo_state.commit_tag.is_empty() {
            output.string_in_color(
                self.config.local_detached_color,
                &format!(
                    "{}{}",
                    self.config.local_detached_prefix, self.repo_state.commit_tag
                ),
            )?;
        } else {
            output.string_in_color(
                self.config.local_detached_color,
                &format!(
                    "{}{}",
                    self.config.local_detached_prefix, self.repo_state.commit_short_sha
                ),
            )?;
        }

        output.write_str(&self.config.local_branch_name_suffix)?;
        output.add_delimter();
        Ok(())
    }

    fn add_local_commits<W: Write>(&self, output: &mut TerminalOutput<W>) -> fmt::Result {
        let push = self.repo_state.commits_to_push;
        let pull = self.repo_state.commits_to_pull;

        if push > 0 && pull > 0 {
            write!(output, "{pull}")?;
            output.colored_tag(&self.config.local_commits_push_pull_infix)?;
            write!(output, "{push}")?;
            output.add_delimter();
        } else if pull > 0 {
            write!(output, "{pull}")?;
            output.colored_tag(&self.config.local_commits_pull_suffix)?;
            output.add_delimter();
        } else if push > 0 {
            write!(output, "{push}")?;
            output.colored_tag(&self.config.local_commits_push_suffix)?;
            output.add_delimter();
        }
        Ok(())
    }

    fn add_repo_state<W: Write>(&self, output: &mut TerminalOutput<W>) -> fmt::Result {
        add_state_elem(
            output,
            self.repo_state.git_local_repo_changes.index_add,
            &self.config.change_index_add_suffix,
        )?;
        add_state_elem(
            output,
            self.repo_state.git_local_repo_changes.index_del,
            &self.config.change_index_del_suffix,
        )?;
        add_state_elem(
            output,
            self.repo_state.git_local_repo_changes.index_mod,
            &self.config.change_index_mod_suffix,
        )?;
        add_state_elem(
            output,
            self.repo_state.git_local_repo_changes.renamed,
            &self.config.change_renamed_suffix,
        )?;
        output.add_delimter();

        add_state_elem(
            output,
            self.repo_state.git_local_repo_changes.local_del,
            &self.config.change_local_del_suffix,
        )?;
        add_state_elem(
            output,
            self.repo_state.git_local_repo_changes.local_mod,
            &self.config.change_local_mod_suffix,
        )?;
        output.add_delimter();

        add_state_elem(
            output,
            self.repo_state.git_local_repo_changes.local_add,
            &self.config.change_local_add_suffix,
        )?;
        output.add_delimter();

        add_state_elem(
            output,
            self.repo_state.git_local_repo_changes.conflict,
            &self.config.change_conflicted_suffix,
        )?;
        output.add_delimter();

        Ok(())
    }

    fn add_stashes<W: Write>(&self, output: &mut TerminalOutput<W>) -> fmt::Result {
        add_state_elem(
            output,
            self.repo_state.stash_count,
            &self.config.stash_suffix,
        )?;
        Ok(())
    }

    fn show_merge_branch_indicator(&self) -> bool {
        self.config.parts.show_merge_branch_commits_diff
            && !self
                .config
                .merge_branch_ignore_branches
                .contains(&self.repo_state.local_branch)
    }
}

impl fmt::Display for Prompt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = TerminalOutput::new(self.shell, f);

        output.end_color_marker()?;
        if self.config.parts.show_repo_indicator {
            self.add_repo_indicator(&mut output)?;
        }
        if self.show_merge_branch_indicator() {
            self.add_no_tracked_upstream_indicator(&mut output)?;
            self.add_merge_branch_commits(&mut output)?;
        }
        if self.config.parts.show_local_branch {
            self.add_local_branch_name(&mut output)?;
        }
        if self.config.parts.show_commits_to_origin {
            self.add_local_commits(&mut output)?;
        }
        if self.config.parts.show_local_changes_state {
            self.add_repo_state(&mut output)?;
        }
        if self.config.parts.show_stashes {
            self.add_stashes(&mut output)?;
        }

        Ok(())
    }
}

fn add_state_elem<W: Write>(
    output: &mut TerminalOutput<W>,
    state_elem: usize,
    colored_tag: &ColoredTag,
) -> fmt::Result {
    if state_elem > 0 {
        write!(output, "{state_elem}")?;
        output.colored_tag(colored_tag)?;
    }
    Ok(())
}
