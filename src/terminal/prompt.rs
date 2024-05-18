use crate::{config::types::Config, git::types::GitRepoState};
use std::fmt;

use super::{
    end_color_marker, tell_string_in_color,
    types::{BaseColor, Color, ColorIntensity, Shell},
};

pub struct Prompt {
    shell: Shell,
    config: Config,
    repo_state: GitRepoState,
    result: String,
}

impl Prompt {
    pub fn build(shell: Shell, config: Config, repo_state: GitRepoState) -> Prompt {
        let mut prompt = Self {
            shell,
            config,
            repo_state,
            result: end_color_marker(shell),
        };

        if prompt.config.parts.show_repo_indicator {
            prompt.add_repo_indicator();
        }
        if show_merge_branch_indicator(&prompt.config, &prompt.repo_state) {
            prompt.add_no_tracked_upstream_indicator();
            prompt.add_merge_branch_commits();
        }
        if prompt.config.parts.show_local_branch {
            prompt.add_local_branch_name();
        }
        if prompt.config.parts.show_commits_to_origin {
            prompt.add_local_commits();
        }
        if prompt.config.parts.show_local_changes_state {
            prompt.add_repo_state();
        }
        if prompt.config.parts.show_stashes {
            prompt.add_stashes();
        }

        prompt
    }

    fn add_repo_indicator(&mut self) {
        self.result += &self.config.repo_indicator;
        self.result += " ";
    }

    fn add_no_tracked_upstream_indicator(&mut self) {
        if self.repo_state.remote_tracking_branch.is_empty() {
            self.result += &tell_string_in_color(
                self.shell,
                self.config.no_tracked_upstream_string_color,
                &self.config.no_tracked_upstream_string,
            );
            self.result += " ";
            self.result += &tell_string_in_color(
                self.shell,
                self.config.no_tracked_upstream_indicator_color,
                &self.config.no_tracked_upstream_indicator,
            );
            self.result += " ";
        }
    }

    fn add_merge_branch_commits(&mut self) {
        let push = self.repo_state.merge_branch_commits_to_push;
        let pull = self.repo_state.merge_branch_commits_to_pull;

        if push > 0 && pull > 0 {
            self.result += &self.config.merge_branch_commits_indicator;
            self.result += " ";
            self.result += &pull.to_string();
            self.result += &tell_string_in_color(
                self.shell,
                Color {
                    color: BaseColor::Green,
                    intensity: ColorIntensity::Vivid,
                },
                &self.config.merge_branch_commits_both_pull_push,
            );
            self.result += &push.to_string();
            self.result += " ";
        } else if pull > 0 {
            self.result += &self.config.merge_branch_commits_indicator;
            self.result += " ";
            self.result += &tell_string_in_color(
                self.shell,
                Color {
                    color: BaseColor::Green,
                    intensity: ColorIntensity::Vivid,
                },
                &self.config.merge_branch_commits_only_pull,
            );
            self.result += " ";
            self.result += &pull.to_string();
            self.result += " ";
        } else if push > 0 {
            self.result += &self.config.merge_branch_commits_indicator;
            self.result += " ";
            self.result += &tell_string_in_color(
                self.shell,
                Color {
                    color: BaseColor::Green,
                    intensity: ColorIntensity::Vivid,
                },
                &self.config.merge_branch_commits_only_push,
            );
            self.result += " ";
            self.result += &push.to_string();
            self.result += " ";
        }
    }

    fn add_local_branch_name(&mut self) {
        self.result += &self.config.local_branch_name_prefix;

        if !self.repo_state.local_branch.is_empty() {
            self.result += &tell_string_in_color(
                self.shell,
                self.config.local_branch_color,
                &self.repo_state.local_branch,
            );
        } else if !self.repo_state.commit_tag.is_empty() {
            self.result += &tell_string_in_color(
                self.shell,
                self.config.local_detached_color,
                &format!(
                    "{}{}",
                    self.config.local_detached_prefix, self.repo_state.commit_tag
                ),
            );
        } else {
            self.result += &tell_string_in_color(
                self.shell,
                self.config.local_detached_color,
                &format!(
                    "{}{}",
                    self.config.local_detached_prefix, self.repo_state.commit_short_sha
                ),
            );
        }

        self.result += &self.config.local_branch_name_suffix;
        self.result += " ";
    }

    fn add_local_commits(&mut self) {
        let push = self.repo_state.commits_to_push;
        let pull = self.repo_state.commits_to_pull;

        if push > 0 && pull > 0 {
            self.result += &pull.to_string();
            self.result += &tell_string_in_color(
                self.shell,
                self.config.local_commits_push_pull_infix_color,
                &self.config.local_commits_push_pull_infix,
            );
            self.result += &push.to_string();
            self.result += " "
        } else if pull > 0 {
            self.result += &pull.to_string();
            self.result += &tell_string_in_color(
                self.shell,
                self.config.local_commits_pull_suffix_color,
                &self.config.local_commits_pull_suffix,
            );
            self.result += " "
        } else if push > 0 {
            self.result += &push.to_string();
            self.result += &tell_string_in_color(
                self.shell,
                self.config.local_commits_push_suffix_color,
                &self.config.local_commits_push_suffix,
            );
            self.result += " "
        }
    }

    fn add_repo_state(&mut self) {
        let inda = self.repo_state.git_local_repo_changes.index_add;
        let indd = self.repo_state.git_local_repo_changes.index_del;
        let indm = self.repo_state.git_local_repo_changes.index_mod;
        let mv = self.repo_state.git_local_repo_changes.renamed;

        add_state_elem(
            &mut self.result,
            inda,
            self.shell,
            self.config.change_index_add_suffix_color,
            &self.config.change_index_add_suffix,
        );
        add_state_elem(
            &mut self.result,
            indd,
            self.shell,
            self.config.change_index_del_suffix_color,
            &self.config.change_index_del_suffix,
        );
        add_state_elem(
            &mut self.result,
            indm,
            self.shell,
            self.config.change_index_mod_suffix_color,
            &self.config.change_index_mod_suffix,
        );
        add_state_elem(
            &mut self.result,
            mv,
            self.shell,
            self.config.change_renamed_suffix_color,
            &self.config.change_renamed_suffix,
        );
        if inda > 0 || indd > 0 || indm > 0 || mv > 0 {
            self.result += " ";
        }

        let ld = self.repo_state.git_local_repo_changes.local_del;
        let lm = self.repo_state.git_local_repo_changes.local_mod;
        add_state_elem(
            &mut self.result,
            ld,
            self.shell,
            self.config.change_local_del_suffix_color,
            &self.config.change_local_del_suffix,
        );
        add_state_elem(
            &mut self.result,
            lm,
            self.shell,
            self.config.change_local_mod_suffix_color,
            &self.config.change_local_mod_suffix,
        );
        if ld > 0 || lm > 0 {
            self.result += " ";
        }
        let la = self.repo_state.git_local_repo_changes.local_add;
        add_state_elem(
            &mut self.result,
            la,
            self.shell,
            self.config.change_local_add_suffix_color,
            &self.config.change_local_add_suffix,
        );
        if la > 0 {
            self.result += " ";
        }

        let co = self.repo_state.git_local_repo_changes.conflict;
        add_state_elem(
            &mut self.result,
            co,
            self.shell,
            self.config.change_conflicted_suffix_color,
            &self.config.change_conflicted_suffix,
        );
        if co > 0 {
            self.result += " ";
        }
    }

    fn add_stashes(&mut self) {
        let stash_count = self.repo_state.stash_count;
        if stash_count > 0 {
            self.result += &stash_count.to_string();
            self.result += &tell_string_in_color(
                self.shell,
                self.config.stash_suffix_color,
                &self.config.stash_suffix,
            );
            self.result += " "
        }
    }
}

impl fmt::Display for Prompt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.result.trim())
    }
}

fn show_merge_branch_indicator(config: &Config, repo_state: &GitRepoState) -> bool {
    config.parts.show_merge_branch_commits_diff
        && !config
            .merge_branch_ignore_branches
            .contains(&repo_state.local_branch)
}

fn add_state_elem(
    result: &mut String,
    state_elem: usize,
    shell: Shell,
    color: Color,
    letter: &str,
) {
    if state_elem > 0 {
        *result += &state_elem.to_string();
        *result += &tell_string_in_color(shell, color, letter)
    }
}
