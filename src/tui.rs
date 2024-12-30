use crate::{Board, BotDifficulty, Command, WIDTH};
use color_print::cprintln;
use getch::Getch;

lazy_static::lazy_static! {
    static ref GETCH: Getch = Getch::new();
}

pub(crate) fn get_command() -> Command {
    loop {
        let input = GETCH.getch().unwrap();
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
    let input = GETCH.getch().unwrap();
    match input.to_ascii_lowercase() {
        b'n' => return false,
        _ => return true,
    }
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
    cprintln!(
        "<bold,red>q</>uit <bold,green>r</>eset <bold,blue>b</>ot={}",
        bot.as_ref()
    );
}

pub(crate) fn pause() {
    GETCH.getch().ok();
}
