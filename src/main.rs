pub mod config;
pub mod git;
pub mod terminal;

use anyhow::Result;
use clap::Parser;
use terminal::types::Shell;

use crate::{
    config::get_app_config,
    git::{check_in_git_directory, get_git_repo_state},
    terminal::prompt::Prompt,
};

#[derive(Parser)]
#[clap(name = "git-radar-rs", version = clap::crate_version!())]
struct Args {
    #[arg(long)]
    show_config: bool,
    #[arg(value_enum, default_value = "other")]
    shell: Shell,
}

fn main() -> Result<()> {
    let args = Args::parse();

    if args.show_config {
        let config = get_app_config()?;

        println!("{}", toml::to_string(&config)?);

        return Ok(());
    }

    if !check_in_git_directory()? {
        return Ok(());
    }

    let config = get_app_config()?;
    let repo_state = get_git_repo_state()?;
    let prompt = Prompt::new(args.shell, config, repo_state);

    print!("{prompt}");

    Ok(())
}
