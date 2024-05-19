use std::fmt;

use super::types::{Color, Shell};

pub struct TerminalOutput<W> {
    shell: Shell,
    writer: W,
    add_delimiter: bool,
}

impl<W> TerminalOutput<W>
where
    W: fmt::Write,
{
    pub fn new(shell: Shell, writer: W) -> Self {
        Self {
            shell,
            writer,
            add_delimiter: false,
        }
    }

    pub fn string_in_color(&mut self, color: Color, str: &str) -> fmt::Result {
        self.start_color_marker(color)?;
        self.writer.write_str(str)?;
        self.end_color_marker()?;
        Ok(())
    }

    pub fn start_color_marker(&mut self, color: Color) -> fmt::Result {
        if self.add_delimiter {
            self.writer.write_char(' ')?;
            self.add_delimiter = false;
        }
        match self.shell {
            Shell::Tmux => self.writer.write_str(color.tmux_start_code()),
            Shell::None => Ok(()),
            _ => self.apply_shell_markers(color.terminal_start_code()),
        }
    }

    pub fn end_color_marker(&mut self) -> fmt::Result {
        if self.add_delimiter {
            self.writer.write_char(' ')?;
            self.add_delimiter = false;
        }
        match self.shell {
            Shell::Tmux => self.writer.write_str("#[fg=default]"),
            Shell::None => Ok(()),
            _ => self.apply_shell_markers("\x1b[0;39m"),
        }
    }

    pub fn add_delimter(&mut self) {
        self.add_delimiter = true;
    }

    fn apply_shell_markers(&mut self, marker: &str) -> fmt::Result {
        match self.shell {
            Shell::Zsh => write!(self.writer, "%{{{}%}}", marker),
            Shell::Bash => write!(self.writer, "\x01{}\x02", marker),
            _ => self.writer.write_str(marker),
        }
    }
}

impl<W> fmt::Write for TerminalOutput<W>
where
    W: fmt::Write,
{
    fn write_str(&mut self, s: &str) -> fmt::Result {
        if self.add_delimiter {
            self.writer.write_char(' ')?;
            self.add_delimiter = false;
        }
        self.writer.write_str(s)
    }
}
