use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Color {
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

#[derive(ValueEnum, Clone, Copy, Debug)]
#[clap(rename_all = "lower")]
pub enum Shell {
    Bash,
    Zsh,
    Tmux,
    None,
    Other,
}
