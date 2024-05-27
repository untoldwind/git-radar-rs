# git-radar-rs

Rust version of [git-radar](https://github.com/michaeldfallen/git-radar) or [gitHUD](https://github.com/gbataille/gitHUD)

## Install

### Cargo

```sh
cargo install git-radar-rs
```

Note: By default `git-radar-rs` compiles with support for `libgit2` (via: https://github.com/rust-lang/git2-rs). 
If this causes problems use:

```sh
cargo install --no-default-feature git-radar-rs
```
to install a version that uses the `git` command-line (just like the original `git-radar`)

## Setup

To track the remote HEAD correctly it might be necessary to run a
```sh
git remote set-head origin --auto
```
once on the repository (or whenever the remote default branch is changed).

### Bash

Example for a simplistic bash-prompt:

```sh
export PS1="\W\$(git-radar-rs bash)\[\033[0m\]\$ "
```

### Zsh

Example for a simplistic zsh-prompt:
```sh
setopt PROMPT_SUBST
export PROMPT='%1d $(git-radar-rs zsh)$ '
```

### Fish

`git-radar-rs` can be used directly inside the "fish_prompt" function.

### Tmux

Example `.tmux.conf´:
```
set -g status-bg '#222222'
set -g status-fg '#ffffff'
set -g status-right "#{pane_current_command} #(cd '#{pane_current_path}' && git-radar-rs tmux)"
```

