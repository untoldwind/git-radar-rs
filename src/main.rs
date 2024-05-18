use std::error::Error;

pub mod config;
pub mod git;
pub mod process;
pub mod terminal;

use clap::Parser;
use terminal::types::Shell;

use crate::{
    config::get_app_config,
    git::{command::check_in_git_directory, get_git_repo_state},
    terminal::prompt::build_prompt,
};

#[derive(Parser)]
struct Args {
    #[arg(long)]
    show_config: bool,
    #[arg(value_enum, default_value = "other")]
    shell: Shell,
}

fn main() -> Result<(), Box<dyn Error>> {
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
    let prompt = build_prompt(args.shell, &config, &repo_state);

    print!("{}", prompt.trim());

    Ok(())
}
