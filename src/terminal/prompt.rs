use crate::{config::types::Config, git::types::GitRepoState};

use super::{
    end_color_marker, tell_string_in_color,
    types::{Color, ColorIntensity, Shell},
};

pub fn build_prompt(shell: Shell, config: &Config, repo_state: &GitRepoState) -> String {
    let mut result = String::new();

    result += &end_color_marker(shell);

    if config.parts.show_repo_indicator {
        result += &config.repo_indicator;
        result += " ";
    }
    if show_merge_branch_indicator(config, repo_state) {
        result += &add_no_tracked_upstream_indicator(shell, config, repo_state);
        result += &add_merge_branch_commits(shell, config, repo_state);
    }
    if config.parts.show_local_branch {
        result += &add_local_branch_name(shell, config, repo_state);
    }
    if config.parts.show_commits_to_origin {
        result += &add_local_commits(shell, config, repo_state);
    }
    if config.parts.show_local_changes_state {
        result += &add_repo_state(shell, config, repo_state);
    }
    if config.parts.show_stashes {
        result += &add_stashes(shell, config, repo_state);
    }

    result
}

fn show_merge_branch_indicator(config: &Config, repo_state: &GitRepoState) -> bool {
    config.parts.show_merge_branch_commits_diff
        && !config
            .merge_branch_ignore_branches
            .contains(&repo_state.local_branch)
}

fn add_no_tracked_upstream_indicator(
    shell: Shell,
    config: &Config,
    repo_state: &GitRepoState,
) -> String {
    let mut result = String::new();
    if repo_state.remote_tracking_branch.is_empty() {
        result += &tell_string_in_color(
            shell,
            config.no_tracked_upstream_string_color,
            config.no_tracked_upstream_string_intensity,
            &config.no_tracked_upstream_string,
        );
        result += " ";
        result += &tell_string_in_color(
            shell,
            config.no_tracked_upstream_indicator_color,
            config.no_tracked_upstream_indicator_intensity,
            &config.no_tracked_upstream_indicator,
        );
        result += " ";
    }
    result
}

fn add_merge_branch_commits(shell: Shell, config: &Config, repo_state: &GitRepoState) -> String {
    let mut result = String::new();
    let push = repo_state.merge_branch_commits_to_push;
    let pull = repo_state.merge_branch_commits_to_pull;

    if push > 0 && pull > 0 {
        result += &config.merge_branch_commits_indicator;
        result += " ";
        result += &pull.to_string();
        result += &tell_string_in_color(
            shell,
            Color::Green,
            ColorIntensity::Vivid,
            &config.merge_branch_commits_both_pull_push,
        );
        result += &push.to_string();
        result += " ";
    } else if pull > 0 {
        result += &config.merge_branch_commits_indicator;
        result += " ";
        result += &tell_string_in_color(
            shell,
            Color::Green,
            ColorIntensity::Vivid,
            &config.merge_branch_commits_only_pull,
        );
        result += " ";
        result += &pull.to_string();
        result += " ";
    } else if push > 0 {
        result += &config.merge_branch_commits_indicator;
        result += " ";
        result += &tell_string_in_color(
            shell,
            Color::Green,
            ColorIntensity::Vivid,
            &config.merge_branch_commits_only_push,
        );
        result += " ";
        result += &push.to_string();
        result += " ";
    }

    result
}

fn add_local_branch_name(shell: Shell, config: &Config, repo_state: &GitRepoState) -> String {
    let mut result = String::new();

    result += &config.local_branch_name_prefix;

    if !repo_state.local_branch.is_empty() {
        result += &tell_string_in_color(
            shell,
            config.local_branch_color,
            config.local_branch_intensity,
            &repo_state.local_branch,
        );
    } else if !repo_state.commit_tag.is_empty() {
        result += &tell_string_in_color(
            shell,
            config.local_detached_color,
            config.local_detached_intensity,
            &format!("{}{}", config.local_detached_prefix, repo_state.commit_tag),
        );
    } else {
        result += &tell_string_in_color(
            shell,
            config.local_detached_color,
            config.local_detached_intensity,
            &format!(
                "{}{}",
                config.local_detached_prefix, repo_state.commit_short_sha
            ),
        );
    }

    result += &config.local_branch_name_suffix;
    result += " ";

    result
}

fn add_local_commits(shell: Shell, config: &Config, repo_state: &GitRepoState) -> String {
    let mut result = String::new();

    let push = repo_state.commits_to_push;
    let pull = repo_state.commits_to_pull;

    if push > 0 && pull > 0 {
        result += &pull.to_string();
        result += &tell_string_in_color(
            shell,
            config.local_commits_push_pull_infix_color,
            config.local_commits_push_pull_infix_intensity,
            &config.local_commits_push_pull_infix,
        );
        result += &push.to_string();
        result += " "
    } else if pull > 0 {
        result += &pull.to_string();
        result += &tell_string_in_color(
            shell,
            config.local_commits_pull_suffix_color,
            config.local_commits_pull_suffix_intensity,
            &config.local_commits_pull_suffix,
        );
        result += " "
    } else if push > 0 {
        result += &push.to_string();
        result += &tell_string_in_color(
            shell,
            config.local_commits_push_suffix_color,
            config.local_commits_push_suffix_intensity,
            &config.local_commits_push_suffix,
        );
        result += " "
    }

    result
}

fn add_repo_state(shell: Shell, config: &Config, repo_state: &GitRepoState) -> String {
    let mut result = String::new();

    let inda = repo_state.git_local_repo_changes.index_add;
    let indd = repo_state.git_local_repo_changes.index_del;
    let indm = repo_state.git_local_repo_changes.index_mod;
    let mv = repo_state.git_local_repo_changes.renamed;

    add_state_elem(
        &mut result,
        inda,
        shell,
        config.change_index_add_suffix_color,
        config.change_index_add_suffix_intensity,
        &config.change_index_add_suffix,
    );
    add_state_elem(
        &mut result,
        indd,
        shell,
        config.change_index_del_suffix_color,
        config.change_index_del_suffix_intensity,
        &config.change_index_del_suffix,
    );
    add_state_elem(
        &mut result,
        indm,
        shell,
        config.change_index_mod_suffix_color,
        config.change_index_mod_suffix_intensity,
        &config.change_index_mod_suffix,
    );
    add_state_elem(
        &mut result,
        mv,
        shell,
        config.change_renamed_suffix_color,
        config.change_renamed_suffix_intensity,
        &config.change_renamed_suffix,
    );
    if inda > 0 || indd > 0 || indm > 0 || mv > 0 {
        result += " ";
    }

    let ld = repo_state.git_local_repo_changes.local_del;
    let lm = repo_state.git_local_repo_changes.local_mod;
    add_state_elem(
        &mut result,
        ld,
        shell,
        config.change_local_del_suffix_color,
        config.change_local_del_suffix_intensity,
        &config.change_local_del_suffix,
    );
    add_state_elem(
        &mut result,
        lm,
        shell,
        config.change_local_mod_suffix_color,
        config.change_local_mod_suffix_intensity,
        &config.change_local_mod_suffix,
    );
    if ld > 0 || lm > 0 {
        result += " ";
    }
    let la = repo_state.git_local_repo_changes.local_add;
    add_state_elem(
        &mut result,
        la,
        shell,
        config.change_local_add_suffix_color,
        config.change_local_add_suffix_intensity,
        &config.change_local_add_suffix,
    );
    if la > 0 {
        result += " ";
    }

    let co = repo_state.git_local_repo_changes.conflict;
    add_state_elem(
        &mut result,
        co,
        shell,
        config.change_conflicted_suffix_color,
        config.change_conflicted_suffix_intensity,
        &config.change_conflicted_suffix,
    );
    if co > 0 {
        result += " ";
    }

    result
}

fn add_state_elem(
    result: &mut String,
    state_elem: usize,
    shell: Shell,
    color: Color,
    color_intensity: ColorIntensity,
    letter: &str,
) {
    if state_elem > 0 {
        *result += &state_elem.to_string();
        *result += &tell_string_in_color(shell, color, color_intensity, letter)
    }
}

fn add_stashes(shell: Shell, config: &Config, repo_state: &GitRepoState) -> String {
    let mut result = String::new();

    let stash_count = repo_state.stash_count;
    if stash_count > 0 {
        result += &stash_count.to_string();
        result += &tell_string_in_color(
            shell,
            config.stash_suffix_color,
            config.stash_suffix_intensity,
            &config.stash_suffix,
        );
        result += " "
    }

    result
}
