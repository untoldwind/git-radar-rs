use serde::{Deserialize, Serialize};

use crate::terminal::types::{BaseColor, Color, ColorIntensity, ColoredTag};

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

    pub no_tracked_upstream_string: ColoredTag,
    pub no_tracked_upstream_indicator: ColoredTag,

    pub merge_branch_commits_indicator: String,
    pub merge_branch_commits_only_push: ColoredTag,
    pub merge_branch_commits_only_pull: ColoredTag,
    pub merge_branch_commits_both_pull_push: ColoredTag,
    pub merge_branch_ignore_branches: Vec<String>,

    pub local_branch_name_prefix: String,
    pub local_branch_name_suffix: String,
    pub local_detached_prefix: String,
    pub local_branch_color: Color,
    pub local_detached_color: Color,

    pub local_commits_push_suffix: ColoredTag,
    pub local_commits_pull_suffix: ColoredTag,
    pub local_commits_push_pull_infix: ColoredTag,

    pub change_index_add_suffix: ColoredTag,
    pub change_index_mod_suffix: ColoredTag,
    pub change_index_del_suffix: ColoredTag,
    pub change_local_add_suffix: ColoredTag,
    pub change_local_mod_suffix: ColoredTag,
    pub change_local_del_suffix: ColoredTag,
    pub change_renamed_suffix: ColoredTag,
    pub change_conflicted_suffix: ColoredTag,

    pub stash_suffix: ColoredTag,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            parts: Default::default(),

            repo_indicator: "ᚴ".into(),

            no_tracked_upstream_string: ColoredTag {
                tag: "upstream".into(),
                color: Color {
                    color: BaseColor::Red,
                    intensity: ColorIntensity::Vivid,
                },
            },
            no_tracked_upstream_indicator: ColoredTag {
                tag: "\u{26A1}".into(),
                color: Color {
                    color: BaseColor::Red,
                    intensity: ColorIntensity::Vivid,
                },
            },

            merge_branch_commits_indicator: "\u{1D62E}".into(),
            merge_branch_commits_only_push: ColoredTag {
                tag: "\u{2190}".into(),
                color: Color {
                    color: BaseColor::Green,
                    intensity: ColorIntensity::Vivid,
                },
            },
            merge_branch_commits_only_pull: ColoredTag {
                tag: "\u{2192}".into(),
                color: Color {
                    color: BaseColor::Green,
                    intensity: ColorIntensity::Vivid,
                },
            },
            merge_branch_commits_both_pull_push: ColoredTag {
                tag: "\u{21C4}".into(),
                color: Color {
                    color: BaseColor::Green,
                    intensity: ColorIntensity::Vivid,
                },
            },
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

            local_commits_push_suffix: ColoredTag {
                tag: "\u{2191}".into(),
                color: Color {
                    color: BaseColor::Green,
                    intensity: ColorIntensity::Vivid,
                },
            },
            local_commits_pull_suffix: ColoredTag {
                tag: "\u{2193}".into(),
                color: Color {
                    color: BaseColor::Red,
                    intensity: ColorIntensity::Vivid,
                },
            },
            local_commits_push_pull_infix: ColoredTag {
                tag: "⥯".into(),
                color: Color {
                    color: BaseColor::Green,
                    intensity: ColorIntensity::Vivid,
                },
            },

            change_index_add_suffix: ColoredTag {
                tag: "A".into(),
                color: Color {
                    color: BaseColor::Green,
                    intensity: ColorIntensity::Vivid,
                },
            },
            change_index_mod_suffix: ColoredTag {
                tag: "M".into(),
                color: Color {
                    color: BaseColor::Green,
                    intensity: ColorIntensity::Vivid,
                },
            },
            change_index_del_suffix: ColoredTag {
                tag: "D".into(),
                color: Color {
                    color: BaseColor::Green,
                    intensity: ColorIntensity::Vivid,
                },
            },
            change_local_add_suffix: ColoredTag {
                tag: "A".into(),
                color: Color {
                    color: BaseColor::White,
                    intensity: ColorIntensity::Vivid,
                },
            },
            change_local_mod_suffix: ColoredTag {
                tag: "M".into(),
                color: Color {
                    color: BaseColor::Red,
                    intensity: ColorIntensity::Vivid,
                },
            },
            change_local_del_suffix: ColoredTag {
                tag: "D".into(),
                color: Color {
                    color: BaseColor::Red,
                    intensity: ColorIntensity::Vivid,
                },
            },
            change_renamed_suffix: ColoredTag {
                tag: "R".into(),
                color: Color {
                    color: BaseColor::Green,
                    intensity: ColorIntensity::Vivid,
                },
            },
            change_conflicted_suffix: ColoredTag {
                tag: "C".into(),
                color: Color {
                    color: BaseColor::Green,
                    intensity: ColorIntensity::Vivid,
                },
            },

            stash_suffix: ColoredTag {
                tag: "≡".into(),
                color: Color {
                    color: BaseColor::Green,
                    intensity: ColorIntensity::Vivid,
                },
            },
        }
    }
}
