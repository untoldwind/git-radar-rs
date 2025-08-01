use self::types::{Color, Shell};

pub mod output;
pub mod prompt;
pub mod types;

pub fn tell_string_in_color(shell: Shell, color: Color, str: &str) -> String {
    format!(
        "{}{}{}",
        start_color_marker(shell, color),
        str,
        end_color_marker(shell)
    )
}

fn start_color_marker(shell: Shell, color: Color) -> String {
    match shell {
        Shell::Tmux => color.tmux_start_code().into(),
        Shell::None => "".into(),
        _ => apply_shell_markers(shell, color.terminal_start_code()),
    }
}

fn end_color_marker(shell: Shell) -> String {
    match shell {
        Shell::Tmux => "#[fg=default]".into(),
        Shell::None => "".into(),
        _ => apply_shell_markers(shell, "\x1b[0;39m"),
    }
}

fn apply_shell_markers(shell: Shell, marker: &str) -> String {
    match shell {
        Shell::Zsh => format!("%{{{marker}%}}"),
        Shell::Bash => format!("\x01{marker}\x02"),
        _ => marker.into(),
    }
}
