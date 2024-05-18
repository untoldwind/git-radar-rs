use serde::{Deserialize, Serialize};

use crate::terminal::types::{BaseColor, Color, ColorIntensity};

#[derive(Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct Parts {
    pub show_repo_indicator: bool,
    pub show_merge_branch_commits_diff: bool,
    pub show_local_branch: bool,
    pub show_commits_to_origin: bool,
    pub show_local_changes_state: bool,
    pub show_stashes: bool,
}

impl Default for Parts {
    fn default() -> Self {
        Self {
            show_repo_indicator: true,
            show_merge_branch_commits_diff: true,
            show_local_branch: true,
            show_commits_to_origin: true,
            show_local_changes_state: true,
            show_stashes: true,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct Config {
    pub parts: Parts,

    pub repo_indicator: String,

    pub no_tracked_upstream_string: String,
    pub no_tracked_upstream_string_color: Color,
    pub no_tracked_upstream_indicator: String,
    pub no_tracked_upstream_indicator_color: Color,

    pub merge_branch_commits_indicator: String,
    pub merge_branch_commits_only_push: String,
    pub merge_branch_commits_only_pull: String,
    pub merge_branch_commits_both_pull_push: String,
    pub merge_branch_ignore_branches: Vec<String>,

    pub local_branch_name_prefix: String,
    pub local_branch_name_suffix: String,
    pub local_detached_prefix: String,
    pub local_branch_color: Color,
    pub local_detached_color: Color,

    pub local_commits_push_suffix: String,
    pub local_commits_push_suffix_color: Color,
    pub local_commits_pull_suffix: String,
    pub local_commits_pull_suffix_color: Color,
    pub local_commits_push_pull_infix: String,
    pub local_commits_push_pull_infix_color: Color,

    pub change_index_add_suffix: String,
    pub change_index_add_suffix_color: Color,
    pub change_index_mod_suffix: String,
    pub change_index_mod_suffix_color: Color,
    pub change_index_del_suffix: String,
    pub change_index_del_suffix_color: Color,
    pub change_local_add_suffix: String,
    pub change_local_add_suffix_color: Color,
    pub change_local_mod_suffix: String,
    pub change_local_mod_suffix_color: Color,
    pub change_local_del_suffix: String,
    pub change_local_del_suffix_color: Color,
    pub change_renamed_suffix: String,
    pub change_renamed_suffix_color: Color,
    pub change_conflicted_suffix: String,
    pub change_conflicted_suffix_color: Color,

    pub stash_suffix: String,
    pub stash_suffix_color: Color,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            parts: Default::default(),

            repo_indicator: "ᚴ".into(),

            no_tracked_upstream_string: "upstream".into(),
            no_tracked_upstream_string_color: Color {
                color: BaseColor::Red,
                intensity: ColorIntensity::Vivid,
            },
            no_tracked_upstream_indicator: "\u{26A1}".into(),
            no_tracked_upstream_indicator_color: Color {
                color: BaseColor::Red,
                intensity: ColorIntensity::Vivid,
            },

            merge_branch_commits_indicator: "\u{1D62E}".into(),
            merge_branch_commits_only_push: "\u{2190}".into(),
            merge_branch_commits_only_pull: "\u{2192}".into(),
            merge_branch_commits_both_pull_push: "\u{21C4}".into(),
            merge_branch_ignore_branches: ["gh-pages".into()].into(),

            local_branch_name_prefix: "[".into(),
            local_branch_name_suffix: "]".into(),
            local_detached_prefix: "detached@".into(),
            local_branch_color: Color {
                color: BaseColor::NoColor,
                intensity: ColorIntensity::Vivid,
            },
            local_detached_color: Color {
                color: BaseColor::Yellow,
                intensity: ColorIntensity::Vivid,
            },

            local_commits_push_suffix: "\u{2191}".into(),
            local_commits_push_suffix_color: Color {
                color: BaseColor::Green,
                intensity: ColorIntensity::Vivid,
            },
            local_commits_pull_suffix: "\u{2193}".into(),
            local_commits_pull_suffix_color: Color {
                color: BaseColor::Red,
                intensity: ColorIntensity::Vivid,
            },
            local_commits_push_pull_infix: "⥯".into(),
            local_commits_push_pull_infix_color: Color {
                color: BaseColor::Green,
                intensity: ColorIntensity::Vivid,
            },

            change_index_add_suffix: "A".into(),
            change_index_add_suffix_color: Color {
                color: BaseColor::Green,
                intensity: ColorIntensity::Vivid,
            },
            change_index_mod_suffix: "M".into(),
            change_index_mod_suffix_color: Color {
                color: BaseColor::Green,
                intensity: ColorIntensity::Vivid,
            },
            change_index_del_suffix: "D".into(),
            change_index_del_suffix_color: Color {
                color: BaseColor::Green,
                intensity: ColorIntensity::Vivid,
            },
            change_local_add_suffix: "A".into(),
            change_local_add_suffix_color: Color {
                color: BaseColor::White,
                intensity: ColorIntensity::Vivid,
            },
            change_local_mod_suffix: "M".into(),
            change_local_mod_suffix_color: Color {
                color: BaseColor::Red,
                intensity: ColorIntensity::Vivid,
            },
            change_local_del_suffix: "D".into(),
            change_local_del_suffix_color: Color {
                color: BaseColor::Red,
                intensity: ColorIntensity::Vivid,
            },
            change_renamed_suffix: "R".into(),
            change_renamed_suffix_color: Color {
                color: BaseColor::Green,
                intensity: ColorIntensity::Vivid,
            },
            change_conflicted_suffix: "C".into(),
            change_conflicted_suffix_color: Color {
                color: BaseColor::Green,
                intensity: ColorIntensity::Vivid,
            },

            stash_suffix: "≡".into(),
            stash_suffix_color: Color {
                color: BaseColor::Green,
                intensity: ColorIntensity::Vivid,
            },
        }
    }
}
