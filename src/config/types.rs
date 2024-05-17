use serde::{Deserialize, Serialize};

use crate::terminal::types::{Color, ColorIntensity};

#[derive(Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct Config {
    show_poart_repo_indicator: bool,
    show_part_merge_branch_commits_diff: bool,
    show_part_local_branch: bool,
    show_part_commits_to_origin: bool,
    show_part_local_changes_state: bool,
    show_part_stashes: bool,

    repo_indicator: String,

    no_tracked_upstream_string: String,
    no_tracked_upstream_string_color: Color,
    no_tracked_upstream_string_intensity: ColorIntensity,
    no_tracked_upstream_indicator: String,
    no_tracked_upstream_indicator_color: Color,
    no_tracked_upstream_indicator_intensity: ColorIntensity,

    merge_branch_commits_indicator: String,
    merge_branch_commits_only_push: String,
    merge_branch_commits_only_pull: String,
    merge_branch_commits_both_pull_push: String,
    merge_branch_ignore_branches: Vec<String>,

    local_branch_name_prefix: String,
    local_branch_name_suffix: String,
    local_detached_prefix: String,
    local_branch_color: Color,
    local_branch_intensity: ColorIntensity,
    local_detached_color: Color,
    local_detached_intensity: ColorIntensity,

    local_commits_push_suffix: String,
    local_commits_push_suffix_color: Color,
    local_commits_push_suffix_intensity: ColorIntensity,
    local_commits_pull_suffix: String,
    local_commits_pull_suffix_color: Color,
    local_commits_pull_suffix_intensity: ColorIntensity,
    local_commits_push_pull_infix: String,
    local_commits_push_pull_infix_color: Color,
    local_commits_push_pull_infix_intensity: ColorIntensity,

    change_index_add_suffix: String,
    change_index_add_suffix_color: Color,
    change_index_add_suffix_intensity: ColorIntensity,
    change_index_mod_suffix: String,
    change_index_mod_suffix_color: Color,
    change_index_mod_suffix_intensity: ColorIntensity,
    change_index_del_suffix: String,
    change_index_del_suffix_color: Color,
    change_index_del_suffix_intensity: ColorIntensity,
    change_local_add_suffix: String,
    change_local_add_suffix_color: Color,
    change_local_add_suffix_intensity: ColorIntensity,
    change_local_mod_suffix: String,
    change_local_mod_suffix_color: Color,
    change_local_mod_suffix_intensity: ColorIntensity,
    change_local_del_suffix: String,
    change_local_del_suffix_color: Color,
    change_local_del_suffix_intensity: ColorIntensity,
    change_renamed_suffix: String,
    change_renamed_suffix_color: Color,
    change_renamed_suffix_intensity: ColorIntensity,
    change_conflicted_suffix: String,
    change_conflicted_suffix_color: Color,
    change_conflicted_suffix_intensity: ColorIntensity,

    stash_suffix: String,
    stash_suffix_color: Color,
    stash_suffix_intensity: ColorIntensity,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            show_poart_repo_indicator: true,
            show_part_merge_branch_commits_diff: true,
            show_part_local_branch: true,
            show_part_commits_to_origin: true,
            show_part_local_changes_state: true,
            show_part_stashes: true,

            repo_indicator: "ᚴ".into(),

            no_tracked_upstream_string: "upstream".into(),
            no_tracked_upstream_string_color: Color::Red,
            no_tracked_upstream_string_intensity: ColorIntensity::Vivid,
            no_tracked_upstream_indicator: "\u{26A1}".into(),
            no_tracked_upstream_indicator_color: Color::Red,
            no_tracked_upstream_indicator_intensity: ColorIntensity::Vivid,

            merge_branch_commits_indicator: "\u{1D62E}".into(),
            merge_branch_commits_only_push: "\u{2190}".into(),
            merge_branch_commits_only_pull: "\u{2192}".into(),
            merge_branch_commits_both_pull_push: "\u{21C4}".into(),
            merge_branch_ignore_branches: ["gh-pages".into()].into(),

            local_branch_name_prefix: "[".into(),
            local_branch_name_suffix: "]".into(),
            local_detached_prefix: "detached@".into(),
            local_branch_color: Color::NoColor,
            local_branch_intensity: ColorIntensity::Vivid,
            local_detached_color: Color::Yellow,
            local_detached_intensity: ColorIntensity::Vivid,

            local_commits_push_suffix: "\u{2191}".into(),
            local_commits_push_suffix_color: Color::Green,
            local_commits_push_suffix_intensity: ColorIntensity::Vivid,
            local_commits_pull_suffix: "\u{2193}".into(),
            local_commits_pull_suffix_color: Color::Red,
            local_commits_pull_suffix_intensity: ColorIntensity::Vivid,
            local_commits_push_pull_infix: "⥯".into(),
            local_commits_push_pull_infix_color: Color::Green,
            local_commits_push_pull_infix_intensity: ColorIntensity::Vivid,

            change_index_add_suffix: "A".into(),
            change_index_add_suffix_color: Color::Green,
            change_index_add_suffix_intensity: ColorIntensity::Vivid,
            change_index_mod_suffix: "M".into(),
            change_index_mod_suffix_color: Color::Green,
            change_index_mod_suffix_intensity: ColorIntensity::Vivid,
            change_index_del_suffix: "D".into(),
            change_index_del_suffix_color: Color::Green,
            change_index_del_suffix_intensity: ColorIntensity::Vivid,
            change_local_add_suffix: "A".into(),
            change_local_add_suffix_color: Color::White,
            change_local_add_suffix_intensity: ColorIntensity::Vivid,
            change_local_mod_suffix: "M".into(),
            change_local_mod_suffix_color: Color::Red,
            change_local_mod_suffix_intensity: ColorIntensity::Vivid,
            change_local_del_suffix: "D".into(),
            change_local_del_suffix_color: Color::Red,
            change_local_del_suffix_intensity: ColorIntensity::Vivid,
            change_renamed_suffix: "R".into(),
            change_renamed_suffix_color: Color::Green,
            change_renamed_suffix_intensity: ColorIntensity::Vivid,
            change_conflicted_suffix: "C".into(),
            change_conflicted_suffix_color: Color::Green,
            change_conflicted_suffix_intensity: ColorIntensity::Vivid,

            stash_suffix: "≡".into(),
            stash_suffix_color: Color::Green,
            stash_suffix_intensity: ColorIntensity::Vivid,
        }
    }
}
