use color_print::cprintln;

use crate::{
    bot, tui, Board, BotDifficulty, CellState, Command, GameOver, DIRECTIONS, I_HEIGHT, I_WIDTH,
    SIZE, WIDTH,
};

pub fn start_game() {
    let mut board = create_board();
    let mut turn: CellState = CellState::PlayerOne;
    let mut bot_difficulty: BotDifficulty = BotDifficulty::Off;

    loop {
        tui::print_ui(&board, bot_difficulty);

        match tui::get_command() {
            Command::Quit => break,
            Command::Bot => {
                bot_difficulty =
                    BotDifficulty::from_repr(bot_difficulty as u8 + 1).unwrap_or_default()
            }
            Command::Reset => reset_game(&mut board, &mut turn),
            Command::Move(row) => match make_move(&mut board, turn, row) {
                Ok(index) => {
                    match check_if_game_over(&board, &turn, index) {
                        Some(state) => {
                            tui::print_ui(&board, bot_difficulty);

                            if handle_game_over(state, bot_difficulty) {
                                reset_game(&mut board, &mut turn);
                            } else {
                                break;
                            }
                        }
                        None => {
                            if bot_difficulty != BotDifficulty::Off {
                                let index = bot::bot_move(&mut board, bot_difficulty);
                                if let Some(state) =
                                    check_if_game_over(&board, &CellState::PlayerTwo, index)
                                {
                                    tui::print_ui(&board, bot_difficulty);

                                    if handle_game_over(state, bot_difficulty) {
                                        reset_game(&mut board, &mut turn);
                                    } else {
                                        break;
                                    }
                                }
                            } else {
                                turn = if turn == CellState::PlayerOne {
                                    CellState::PlayerTwo
                                } else {
                                    CellState::PlayerOne
                                };
                            }
                        }
                    };
                }
                Err(message) => {
                    cprintln!("<red>{}</>", message);
                    println!("Press any key to continue...");
                    tui::pause();
                }
            },
        }
    }
}

fn transform_to_index(row: usize, column: usize) -> usize {
    column * WIDTH + row
}

pub(crate) fn check_if_game_over(
    board: &[CellState; SIZE],
    turn: &CellState,
    last_move_index: usize,
) -> Option<GameOver> {
    let r = last_move_index as isize % I_WIDTH;
    let c = last_move_index as isize / I_WIDTH;

    for (row_step, column_step) in DIRECTIONS {
        let mut count = 1;

        let mut row_delta = r + row_step;
        let mut col_delta = c + column_step;

        // forwards
        while row_delta >= 0 && row_delta < I_WIDTH && col_delta >= 0 && col_delta < I_HEIGHT - 1 {
            let i = transform_to_index(row_delta as usize, col_delta as usize);

            if board[i] != *turn {
                break;
            };

            count += 1;

            row_delta += row_step;
            col_delta += column_step;
        }

        let mut row_delta = r - row_step;
        let mut col_delta = c - column_step;

        // backwards
        while row_delta >= 0 && row_delta < I_WIDTH && col_delta >= 0 && col_delta < I_HEIGHT - 1 {
            let i = transform_to_index(row_delta as usize, col_delta as usize);

            if board[i] != *turn {
                break;
            };

            count += 1;

            row_delta -= row_step;
            col_delta -= column_step;
        }

        if count >= 4 {
            return Some(GameOver::Winner(*turn));
        }
    }

    // Check for draw
    if board.iter().all(|&cell| cell != CellState::Empty) {
        return Some(GameOver::Draw);
    }

    None
}

pub(crate) fn get_last_in_row(board: &Board, row: usize) -> Result<usize, String> {
    if row >= WIDTH {
        return Err("Invalid row!".to_string());
    }

    for i in (row..SIZE).step_by(WIDTH).rev() {
        if board[i] == CellState::Empty {
            return Ok(i);
        }
    }

    return Err("Row has no space!".to_string());
}

fn reset_game(board: &mut Board, turn: &mut CellState) {
    board.fill(CellState::Empty);
    *turn = CellState::PlayerOne;
}

pub(crate) fn create_board() -> Board {
    [CellState::Empty; SIZE]
}

fn make_move(board: &mut Board, turn: CellState, row: usize) -> Result<usize, String> {
    match get_last_in_row(&board, row) {
        Ok(index) => {
            board[index] = turn;
            return Ok(index);
        }
        Err(message) => return Err(message),
    }
}

fn handle_game_over(state: GameOver, bot_difficulty: BotDifficulty) -> bool {
    match state {
        GameOver::Draw => println!("It's a draw!"),
        GameOver::Winner(cell_state) => match cell_state {
            CellState::Empty => unreachable!(),
            CellState::PlayerOne => println!("Player one wins!"),
            CellState::PlayerTwo => {
                if bot_difficulty == BotDifficulty::Off {
                    println!("Player two wins!")
                } else {
                    println!("The bot wins!")
                }
            }
        },
    };

    println!("Reset? [Yn]");
    return tui::confirm();
}

#[cfg(test)]
mod test {
    use crate::SIZE;

    use super::{check_if_game_over, create_board, CellState};

    #[test]
    fn test_game_over_1() {
        let mut board = create_board();
        board[10] = CellState::PlayerTwo;
        board[16] = CellState::PlayerOne;
        board[17] = CellState::PlayerOne;
        board[21] = CellState::PlayerOne;
        board[24] = CellState::PlayerOne;
        board[23] = CellState::PlayerTwo;
        board[28] = CellState::PlayerTwo;
        board[29] = CellState::PlayerTwo;
        board[30] = CellState::PlayerOne;
        board[31] = CellState::PlayerTwo;
        board[32] = CellState::PlayerOne;
        board[35] = CellState::PlayerTwo;
        board[36] = CellState::PlayerOne;
        board[37] = CellState::PlayerTwo;
        board[38] = CellState::PlayerOne;
        board[39] = CellState::PlayerOne;
        board[40] = CellState::PlayerTwo;
        board[41] = CellState::PlayerTwo;

        let game_over = check_if_game_over(&board, &CellState::PlayerTwo, 41);
        assert!(game_over.is_none());
    }

    #[test]
    fn test_game_over_2() {
        let mut board = create_board();
        board[3] = CellState::PlayerTwo;
        board[10] = CellState::PlayerTwo;
        board[16] = CellState::PlayerTwo;
        board[17] = CellState::PlayerOne;
        board[23] = CellState::PlayerTwo;
        board[24] = CellState::PlayerOne;
        board[25] = CellState::PlayerOne;
        board[29] = CellState::PlayerTwo;
        board[30] = CellState::PlayerOne;
        board[31] = CellState::PlayerTwo;
        board[32] = CellState::PlayerOne;
        board[34] = CellState::PlayerOne;
        board[35] = CellState::PlayerTwo;
        board[36] = CellState::PlayerOne;
        board[37] = CellState::PlayerOne;
        board[38] = CellState::PlayerOne;
        board[39] = CellState::PlayerTwo;
        board[41] = CellState::PlayerTwo;

        for i in 0..SIZE {
            if board[i] != CellState::Empty {
                let game_over = check_if_game_over(&board, &board[i], i);
                assert!(game_over.is_none(), "index: {}, winner: {:?}", i, game_over);
            }
        }
    }
}
