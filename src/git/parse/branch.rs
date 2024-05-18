pub fn build_fully_qualified_remote_branch_name(remote: &str, remote_branch_name: &str) -> String {
    format!(
        "{}/{}",
        remote,
        simple_remote_branch_name(remote_branch_name)
    )
}

fn simple_remote_branch_name(remote_branch_name: &str) -> &str {
    if let Some(branch) = remote_branch_name.strip_prefix("refs/heads/") {
        branch
    } else {
        remote_branch_name
    }
}
