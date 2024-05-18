use anyhow::Result;
use std::str;

use super::process::process_with_ignore_exit_code;

pub fn git_cmd_local_branch_name() -> Result<String> {
    Ok(str::from_utf8(&process_with_ignore_exit_code(
        "git",
        &["symbolic-ref", "--short", "HEAD"],
    )?)?
    .trim_end()
    .into())
}

pub fn git_cmd_merge_base(local_branch_name: &str) -> Result<String> {
    Ok(str::from_utf8(&process_with_ignore_exit_code(
        "git",
        &["merge-base", "origin/master", local_branch_name],
    )?)?
    .trim_end()
    .into())
}

pub fn git_cmd_remote_name(local_branch_name: &str) -> Result<String> {
    Ok(str::from_utf8(&process_with_ignore_exit_code(
        "git",
        &[
            "config",
            "--get",
            &git_remote_tracking_config_key(local_branch_name),
        ],
    )?)?
    .trim_end()
    .into())
}

pub fn git_cmd_remote_branch_name(local_branch_name: &str) -> Result<String> {
    Ok(str::from_utf8(&process_with_ignore_exit_code(
        "git",
        &[
            "config",
            "--get",
            &git_remote_branch_config_key(local_branch_name),
        ],
    )?)?
    .trim_end()
    .into())
}

pub fn git_cmd_porcelain_status() -> Result<Vec<u8>> {
    process_with_ignore_exit_code("git", &["status", "--porcelain"])
}

pub fn git_cmd_rev_to_push(from_commit: &str, to_commit: &str) -> Result<usize> {
    Ok(str::from_utf8(&process_with_ignore_exit_code(
        "git",
        &[
            "rev-list",
            "--no-merges",
            "--right-only",
            "--count",
            &merge_base_diff_from_to(from_commit, to_commit),
        ],
    )?)?
    .trim_end()
    .parse()?)
}

pub fn git_cmd_rev_to_pull(from_commit: &str, to_commit: &str) -> Result<usize> {
    Ok(str::from_utf8(&process_with_ignore_exit_code(
        "git",
        &[
            "rev-list",
            "--no-merges",
            "--left-only",
            "--count",
            &merge_base_diff_from_to(from_commit, to_commit),
        ],
    )?)?
    .trim_end()
    .parse()?)
}

pub fn git_cmd_stash_count() -> Result<usize> {
    Ok(process_with_ignore_exit_code("git", &["stash", "list"])?
        .into_iter()
        .filter(|ch| *ch == 10)
        .count())
}

pub fn git_cmd_commit_short_sha() -> Result<String> {
    Ok(str::from_utf8(&process_with_ignore_exit_code(
        "git",
        &["rev-parse", "--short", "HEAD"],
    )?)?
    .trim_end()
    .into())
}

pub fn git_cmd_commit_tag() -> Result<String> {
    Ok(str::from_utf8(&process_with_ignore_exit_code(
        "git",
        &["describe", "--exact-match", "--tags"],
    )?)?
    .trim_end()
    .into())
}

fn git_remote_tracking_config_key(local_branch_name: &str) -> String {
    format!("branch.{}.remote", local_branch_name)
}

fn git_remote_branch_config_key(local_branch_name: &str) -> String {
    format!("branch.{}.merge", local_branch_name)
}

fn merge_base_diff_from_to(from_commit: &str, to_commit: &str) -> String {
    format!("{}...{}", from_commit, to_commit)
}
