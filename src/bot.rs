use crate::{game, Board, BotDifficulty, CellState, WIDTH};
use rand::prelude::*;

pub(crate) fn bot_move(board: &mut Board, difficulty: BotDifficulty) -> usize {
    let best_move = get_best_move(&board, difficulty);
    board[best_move] = CellState::PlayerTwo;
    return best_move;
}

fn get_best_move(board: &Board, difficulty: BotDifficulty) -> usize {
    // count is always uneven, but div by 2 floors the number
    let count_moves = board.iter().filter(|c| c != &&CellState::Empty).count() / 2;

    match count_moves {
        // first move; place on top hehe >:3
        0 => {
            board
                .iter()
                .position(|cell| cell == &CellState::PlayerOne)
                .unwrap()
                - WIDTH
        }
        _ => {
            let mut scores = vec![];

            for row in 0..WIDTH {
                if let Ok(index) = game::get_last_in_row(&board, row) {
                    scores.push((
                        index,
                        recursive_check_outcome(board.clone(), index, CellState::PlayerTwo, 0),
                    ));
                }
            }

            scores.sort_by(|a, b| b.1.total_cmp(&a.1));

            // can just win?
            if scores[0].1 == 1.0 {
                return scores[0].0;
            };

            let mut rng = rand::thread_rng();
            let length = scores.len();

            match difficulty {
                BotDifficulty::Easy => scores[rng.gen_range(0..length)].0,
                BotDifficulty::Normal => {
                    scores[rng.gen_range(0..(length.saturating_sub(length / 2).max(1)))].0
                }
                BotDifficulty::Difficult => {
                    scores[rng.gen_range(0..(length.saturating_sub(length / 4).max(1)))].0
                }
                BotDifficulty::Expert => scores[0].0,
                BotDifficulty::Off => unreachable!(),
            }
        }
    }
}

fn recursive_check_outcome(
    mut board: Board,
    index: usize,
    cell_state: CellState,
    depth: usize,
) -> f32 {
    board[index] = cell_state;

    let game_over = game::check_if_game_over(&board, &cell_state, index);

    match game_over {
        Some(state) => match state {
            crate::GameOver::Draw => 1.0,
            crate::GameOver::Winner(CellState::PlayerOne) => 0.0,
            crate::GameOver::Winner(CellState::PlayerTwo) => 1.0,
            crate::GameOver::Winner(_) => unreachable!(),
        },
        None => {
            let mut score = 1.0;

            if depth == 5 {
                return score;
            }

            for row in 0..WIDTH {
                if let Ok(index) = game::get_last_in_row(&board, row) {
                    let nested_score = recursive_check_outcome(
                        board.clone(),
                        index,
                        if cell_state == CellState::PlayerOne {
                            CellState::PlayerTwo
                        } else {
                            CellState::PlayerOne
                        },
                        depth + 1,
                    );
                    score = (score + nested_score) / 2.0;
                }
            }

            score
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{game, tui, SIZE, WIDTH};

    use super::{get_best_move, BotDifficulty, CellState};

    #[test]
    fn test_best_move() {
        let mut board = game::create_board();
        for i in 0..(SIZE / 2) {
            let index = game::get_last_in_row(&mut board, i % WIDTH).unwrap();
            board[index] = CellState::PlayerOne;
            if let Some(_) = game::check_if_game_over(&board, &CellState::PlayerOne, index) {
                println!("Player 1 wins!");
                break;
            }

            let index = get_best_move(&board, BotDifficulty::Easy);
            board[index] = CellState::PlayerTwo;
            tui::print_ui(&board, BotDifficulty::Easy);

            if let Some(_) = game::check_if_game_over(&board, &CellState::PlayerTwo, index) {
                println!("Player 2 wins!");
                break;
            }
        }
    }
}
