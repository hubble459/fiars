use color_print::cstr;
use std::fmt::Display;

pub(crate) mod bot;
pub mod game;
pub(crate) mod tui;

pub const WIDTH: usize = 7;
pub const HEIGHT: usize = 6;

pub const SIZE: usize = WIDTH * HEIGHT;
pub const I_WIDTH: isize = WIDTH as isize;
pub const I_HEIGHT: isize = WIDTH as isize;
pub const I_SIZE: isize = SIZE as isize;
pub const DIRECTIONS: [(isize, isize); 4] = [
    (1, 0),  // Horizontal
    (0, 1),  // Vertical
    (1, 1),  // Diagonal TL-BR
    (-1, 1), // Diagonal TR-BL
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CellState {
    Empty,
    PlayerOne,
    PlayerTwo,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GameOver {
    Draw,
    Winner(CellState),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Command {
    Quit,
    Bot,
    Reset,
    Move(usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, strum::AsRefStr, strum::FromRepr, Default)]
#[repr(u8)]
pub(crate) enum BotDifficulty {
    #[strum(serialize = "off")]
    #[default]
    Off = 0,
    #[strum(serialize = "easy")]
    Easy = 1,
    #[strum(serialize = "normal")]
    Normal = 2,
    #[strum(serialize = "difficult")]
    Difficult = 3,
    #[strum(serialize = "expert")]
    Expert = 4,
}

impl Display for CellState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CellState::Empty => " ",
                CellState::PlayerOne => cstr!("<green>X</green>"),
                CellState::PlayerTwo => cstr!("<red>O</red>"),
            }
        )
    }
}

pub(crate) type Board = [CellState; SIZE];
