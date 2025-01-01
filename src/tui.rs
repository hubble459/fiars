use crate::{Board, BotDifficulty, Command, WIDTH};
use getch::Getch;

pub(crate) fn get_command() -> Command {
    loop {
        let input = read_key();
        match input.to_ascii_lowercase() {
            b'q' => return Command::Quit,
            b'b' => return Command::Bot,
            b'r' => return Command::Reset,
            b'0'..=b'9' => return Command::Move(((input).saturating_sub(b'1')) as usize),
            _ => {}
        }
    }
}

pub(crate) fn confirm() -> bool {
    let input = read_key();
    match input.to_ascii_lowercase() {
        b'n' => return false,
        _ => return true,
    }
}

fn read_key() -> u8 {
    Getch::new().getch().unwrap()
}

fn clear_ui() {
    // clear screen and place cursor on 1,1
    print!("\x1B[2J\x1B[1;1H");
}

pub(crate) fn print_ui(board: &Board, bot: &BotDifficulty) {
    clear_ui();

    print!("|");
    for i in 1..=WIDTH {
        print!("{i}|");
    }
    println!();
    print!("{}", "=".repeat(WIDTH * 2 + 1));

    for (i, cell) in board.iter().enumerate() {
        if i % WIDTH == 0 {
            println!();
            print!("|");
        }

        print!("{cell}|");
    }

    println!();
    println!("{}", "=".repeat(WIDTH * 2 + 1));
    println!(
        "\x1b[1;31mq\x1b[0muit \x1b[1;32mr\x1b[0meset \x1b[1;34mb\x1b[0mot={}",
        bot
    );
}

pub(crate) fn pause() {
    Getch::new().getch().ok();
}
