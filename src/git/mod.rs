pub mod branch;
#[cfg(not(feature = "libgit"))]
pub mod cli;
#[cfg(feature = "libgit")]
pub mod libgit;
pub mod types;

#[cfg(not(feature = "libgit"))]
pub use cli::{check_in_git_directory, get_git_repo_state};
#[cfg(feature = "libgit")]
pub use libgit::{check_in_git_directory, get_git_repo_state};
