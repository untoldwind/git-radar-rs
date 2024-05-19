use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum BaseColor {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    NoColor,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum ColorIntensity {
    Dull,
    Vivid,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub struct Color {
    pub color: BaseColor,
    pub intensity: ColorIntensity,
}

impl Color {
    pub fn terminal_start_code(&self) -> &'static str {
        match (self.color, self.intensity) {
            (BaseColor::Black, ColorIntensity::Vivid) => "\x1b[1;30m",
            (BaseColor::Red, ColorIntensity::Vivid) => "\x1b[1;31m",
            (BaseColor::Green, ColorIntensity::Vivid) => "\x1b[1;32m",
            (BaseColor::Yellow, ColorIntensity::Vivid) => "\x1b[1;33m",
            (BaseColor::Blue, ColorIntensity::Vivid) => "\x1b[1;34m",
            (BaseColor::Magenta, ColorIntensity::Vivid) => "\x1b[1;35m",
            (BaseColor::Cyan, ColorIntensity::Vivid) => "\x1b[1;36m",
            (BaseColor::White, ColorIntensity::Vivid) => "\x1b[1;37m",
            (BaseColor::Black, ColorIntensity::Dull) => "\x1b[30m",
            (BaseColor::Red, ColorIntensity::Dull) => "\x1b[31m",
            (BaseColor::Green, ColorIntensity::Dull) => "\x1b[32m",
            (BaseColor::Yellow, ColorIntensity::Dull) => "\x1b[33m",
            (BaseColor::Blue, ColorIntensity::Dull) => "\x1b[34m",
            (BaseColor::Magenta, ColorIntensity::Dull) => "\x1b[35m",
            (BaseColor::Cyan, ColorIntensity::Dull) => "\x1b[36m",
            (BaseColor::White, ColorIntensity::Dull) => "\x1b[37m",
            (BaseColor::NoColor, _) => "\x1b[0;39m",
        }
    }

    pub fn tmux_start_code(&self) -> &'static str {
        match (self.color, self.intensity) {
            (BaseColor::Black, ColorIntensity::Vivid) => "#[fg=brightblack]",
            (BaseColor::Red, ColorIntensity::Vivid) => "#[fg=brightred]",
            (BaseColor::Green, ColorIntensity::Vivid) => "#[fg=brightgreen]",
            (BaseColor::Yellow, ColorIntensity::Vivid) => "#[fg=brightyellow]",
            (BaseColor::Blue, ColorIntensity::Vivid) => "#[fg=brightblue]",
            (BaseColor::Magenta, ColorIntensity::Vivid) => "#[fg=brightmagenta]",
            (BaseColor::Cyan, ColorIntensity::Vivid) => "#[fg=brightcyan]",
            (BaseColor::White, ColorIntensity::Vivid) => "#[fg=brightwhite]",
            (BaseColor::Black, ColorIntensity::Dull) => "#[fg=black]",
            (BaseColor::Red, ColorIntensity::Dull) => "#[fg=red]",
            (BaseColor::Green, ColorIntensity::Dull) => "#[fg=green]",
            (BaseColor::Yellow, ColorIntensity::Dull) => "#[fg=yellow]",
            (BaseColor::Blue, ColorIntensity::Dull) => "#[fg=blue]",
            (BaseColor::Magenta, ColorIntensity::Dull) => "#[fg=magenta]",
            (BaseColor::Cyan, ColorIntensity::Dull) => "#[fg=cyan]",
            (BaseColor::White, ColorIntensity::Dull) => "#[fg=white]",
            (BaseColor::NoColor, _) => "#[fg=default]",
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub struct ColoredTag {
    #[serde(flatten)]
    pub color: Color,
    pub tag: String,
}

#[derive(ValueEnum, Clone, Copy, Debug)]
#[clap(rename_all = "lower")]
pub enum Shell {
    Bash,
    Zsh,
    Tmux,
    None,
    Other,
}
