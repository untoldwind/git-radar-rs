use self::types::{Color, ColorIntensity, Shell};

pub mod prompt;
pub mod types;

pub fn tell_string_in_color(
    shell: Shell,
    color: Color,
    color_intensity: ColorIntensity,
    str: &str,
) -> String {
    format!(
        "{}{}{}",
        start_color_marker(shell, color, color_intensity),
        str,
        end_color_marker(shell)
    )
}

fn start_color_marker(shell: Shell, color: Color, color_intensity: ColorIntensity) -> String {
    match shell {
        Shell::Tmux => tmux_start_code(color, color_intensity).into(),
        Shell::None => "".into(),
        _ => apply_shell_markers(shell, terminal_start_code(color, color_intensity)),
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
        Shell::Zsh => format!("%{{{}%}}", marker),
        Shell::Bash => format!("\x01{}\x02", marker),
        _ => marker.into(),
    }
}

fn terminal_start_code(color: Color, color_intensity: ColorIntensity) -> &'static str {
    match (color, color_intensity) {
        (Color::Black, ColorIntensity::Vivid) => "\x1b[1;30m",
        (Color::Red, ColorIntensity::Vivid) => "\x1b[1;31m",
        (Color::Green, ColorIntensity::Vivid) => "\x1b[1;32m",
        (Color::Yellow, ColorIntensity::Vivid) => "\x1b[1;33m",
        (Color::Blue, ColorIntensity::Vivid) => "\x1b[1;34m",
        (Color::Magenta, ColorIntensity::Vivid) => "\x1b[1;35m",
        (Color::Cyan, ColorIntensity::Vivid) => "\x1b[1;36m",
        (Color::White, ColorIntensity::Vivid) => "\x1b[1;37m",
        (Color::Black, ColorIntensity::Dull) => "\x1b[30m",
        (Color::Red, ColorIntensity::Dull) => "\x1b[31m",
        (Color::Green, ColorIntensity::Dull) => "\x1b[32m",
        (Color::Yellow, ColorIntensity::Dull) => "\x1b[33m",
        (Color::Blue, ColorIntensity::Dull) => "\x1b[34m",
        (Color::Magenta, ColorIntensity::Dull) => "\x1b[35m",
        (Color::Cyan, ColorIntensity::Dull) => "\x1b[36m",
        (Color::White, ColorIntensity::Dull) => "\x1b[37m",
        (Color::NoColor, _) => "\x1b[0;39m",
    }
}

fn tmux_start_code(color: Color, color_intensity: ColorIntensity) -> &'static str {
    match (color, color_intensity) {
        (Color::Black, ColorIntensity::Vivid) => "#[fg=brightblack]",
        (Color::Red, ColorIntensity::Vivid) => "#[fg=brightred]",
        (Color::Green, ColorIntensity::Vivid) => "#[fg=brightgreen]",
        (Color::Yellow, ColorIntensity::Vivid) => "#[fg=brightyellow]",
        (Color::Blue, ColorIntensity::Vivid) => "#[fg=brightblue]",
        (Color::Magenta, ColorIntensity::Vivid) => "#[fg=brightmagenta]",
        (Color::Cyan, ColorIntensity::Vivid) => "#[fg=brightcyan]",
        (Color::White, ColorIntensity::Vivid) => "#[fg=brightwhite]",
        (Color::Black, ColorIntensity::Dull) => "#[fg=black]",
        (Color::Red, ColorIntensity::Dull) => "#[fg=red]",
        (Color::Green, ColorIntensity::Dull) => "#[fg=green]",
        (Color::Yellow, ColorIntensity::Dull) => "#[fg=yellow]",
        (Color::Blue, ColorIntensity::Dull) => "#[fg=blue]",
        (Color::Magenta, ColorIntensity::Dull) => "#[fg=magenta]",
        (Color::Cyan, ColorIntensity::Dull) => "#[fg=cyan]",
        (Color::White, ColorIntensity::Dull) => "#[fg=white]",
        (Color::NoColor, _) => "#[fg=default]",
    }
}
