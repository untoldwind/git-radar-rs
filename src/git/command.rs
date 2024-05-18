use std::error::Error;
use std::str;

use crate::process::{process_with_exit_code, process_with_ignore_exit_code};

pub fn check_in_git_directory() -> Result<bool, Box<dyn Error>> {
    let (exit_code, _) = process_with_exit_code("git", &["rev-parse", "--git-dir"])?;
    Ok(exit_code.success())
}

pub fn git_cmd_local_branch_name() -> Result<String, Box<dyn Error>> {
    Ok(str::from_utf8(&process_with_ignore_exit_code(
        "git",
        &["symbolic-ref", "--short", "HEAD"],
    )?)?
    .trim_end()
    .into())
}

pub fn git_cmd_merge_base(local_branch_name: &str) -> Result<String, Box<dyn Error>> {
    Ok(str::from_utf8(&process_with_ignore_exit_code(
        "git",
        &["merge-base", "origin/master", local_branch_name],
    )?)?
    .trim_end()
    .into())
}

pub fn git_cmd_remote_name(local_branch_name: &str) -> Result<String, Box<dyn Error>> {
    Ok(str::from_utf8(&process_with_ignore_exit_code(
        "git",
        &["config", "--get", &git_remote_tracking_config_key(local_branch_name)],
    )?)?
    .trim_end()
    .into())
}


pub fn git_cmd_remote_branch_name(local_branch_name: &str) -> Result<String, Box<dyn Error>> {
    Ok(str::from_utf8(&process_with_ignore_exit_code(
        "git",
        &["config", "--get", &git_remote_branch_config_key(local_branch_name)],
    )?)?
    .trim_end()
    .into())
}

pub fn git_cmd_porcelain_status() -> Result<Vec<u8>, Box<dyn Error>> {
    process_with_ignore_exit_code("git", &["status", "--porcelain"])
}

pub fn git_cmd_stash_count() -> Result<usize, Box<dyn Error>> {
    Ok(process_with_ignore_exit_code("git", &["stash", "list"])?.into_iter().filter(|ch| *ch == 10).count())
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