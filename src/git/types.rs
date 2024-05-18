use std::ops;

#[derive(Debug, Clone, Copy)]
pub enum GitFileState {
    LocalMod,
    LocalAdd,
    LocalDel,
    IndexMod,
    IndexAdd,
    IndexDel,
    Renamed,
    Conflict,
    Skip,
}

#[derive(Debug, Default)]
pub struct GitLocalRepoChanges {
    pub local_mod: usize,
    pub local_add: usize,
    pub local_del: usize,
    pub index_mod: usize,
    pub index_add: usize,
    pub index_del: usize,
    pub renamed: usize,
    pub conflict: usize,
}

impl FromIterator<GitFileState> for GitLocalRepoChanges {
    fn from_iter<T: IntoIterator<Item = GitFileState>>(iter: T) -> Self {
        let mut changes = GitLocalRepoChanges::default();
        for state in iter {
            match state {
                GitFileState::LocalMod => changes.local_mod += 1,
                GitFileState::LocalAdd => changes.local_add += 1,
                GitFileState::LocalDel => changes.local_del += 1,
                GitFileState::IndexMod => changes.index_mod += 1,
                GitFileState::IndexAdd => changes.index_add += 1,
                GitFileState::IndexDel => changes.index_del += 1,
                GitFileState::Renamed => changes.renamed += 1,
                GitFileState::Conflict => changes.conflict += 1,
                _ => (),
            }
        }
        changes
    }
}

impl ops::Add for GitLocalRepoChanges {
    type Output = GitLocalRepoChanges;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            local_mod: self.local_mod + rhs.local_mod,
            local_add: self.local_add + rhs.local_add,
            local_del: self.local_del + rhs.local_del,
            index_mod: self.index_mod + rhs.index_mod,
            index_add: self.index_add + rhs.index_add,
            index_del: self.index_del + rhs.index_del,
            renamed: self.renamed + rhs.renamed,
            conflict: self.conflict + rhs.conflict,
        }
    }
}

#[derive(Debug, Default)]
pub struct GitRepoState {
    pub git_local_repo_changes: GitLocalRepoChanges,
    pub local_branch: String,
    pub commit_short_sha: String,
    pub commit_tag: String,
    pub remote: String,
    pub remote_tracking_branch: String,
    pub stash_count: usize,
    pub commits_to_pull: usize,
    pub commits_to_push: usize,
    pub merge_branch_commits_to_pull: usize,
    pub merge_branch_commits_to_push: usize,
}
