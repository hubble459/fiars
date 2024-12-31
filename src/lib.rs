use std::fmt;
use std::fmt::{Display, Formatter};

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

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum CellState {
    Empty,
    PlayerOne,
    PlayerTwo,
}

#[derive(PartialEq, Eq)]
pub(crate) enum GameOver {
    Draw,
    Winner(CellState),
}

#[derive(PartialEq, Eq)]
pub(crate) enum Command {
    Quit,
    Bot,
    Reset,
    Move(usize),
}

#[derive(Default, PartialEq, Eq)]
#[repr(u8)]
pub(crate) enum BotDifficulty {
    #[default]
    Off = 0,
    Easy = 1,
    Normal = 2,
    Difficult = 3,
    Expert = 4,
}

impl Display for BotDifficulty {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                BotDifficulty::Off => "off",
                BotDifficulty::Easy => "easy",
                BotDifficulty::Normal => "normal",
                BotDifficulty::Difficult => "difficult",
                BotDifficulty::Expert => "expert"
            }
        )
    }
}

impl Display for CellState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CellState::Empty => " ",
                CellState::PlayerOne => "\x1b[32mX\x1b[0m",
                CellState::PlayerTwo => "\x1b[31mO\x1b[0m",
            }
        )
    }
}

pub(crate) type Board = [CellState; SIZE];
