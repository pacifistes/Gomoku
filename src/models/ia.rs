use crate::models::gameboard::*;
use crate::eval::*;
use std::collections::HashMap;
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct IA {
    pub depth: u8,
}

impl IA {
    pub fn new(depth: u8) -> IA {
        IA {
            depth,
        }
    }
}

impl IA {
	pub fn expand(&self, state: &Gameboard, stone: u8, depth: u8) -> Vec<Gameboard> {
		let possible_moves: Vec<(usize, usize)> = state.expand();
		let mut possible_boards: Vec<Gameboard> = possible_moves.iter().map(|new_move| {
			let mut new_state = state.clone();
			new_state.result = None;
			new_state.make_move(new_move.0, new_move.1, stone);
			new_state
		}).collect();
		if (stone == BLACK) {
			possible_boards.sort_by(|board, other| {
					board.value.cmp(&other.value)
			});
		}
		else {
			possible_boards.sort_by(|board, other| {
				other.value.cmp(&board.value)
			});
		}
		possible_boards
	}

	pub fn negascout(&self, state: &mut Gameboard, stone: u8, depth: u8, mut alpha: isize, beta: isize, all_values: &mut HashMap<(usize, usize), isize>) -> isize {
		if depth == 0 || state.is_finish() {
			let mut score = state.value;
			score += (state.white_captures as isize * state.white_captures as isize * 100) - (state.black_captures as isize * state.black_captures as isize * 100);
			if stone == BLACK {
				return -score;
			} else {
				return score;
			}
        }
        let mut best_move: Option<(usize, usize)> = None;
        let mut current = (std::i64::MIN + 1) as isize;
		let mut tmp_beta = beta;
		let mut i = 0;
		let possible_states: Vec<Gameboard> = self.expand(state, stone, depth);
        for mut new_state in possible_states {
            let mut score = -self.negascout(&mut new_state, opposite_stone!(stone), depth - 1, -tmp_beta, -alpha, all_values);
            if score > alpha && score < beta && i > 0 && depth > 1 {
                score = -self.negascout(&mut new_state, opposite_stone!(stone), depth - 1, -beta, -alpha, all_values);
            }
			if depth == self.depth {
				all_values.insert((new_state.last_move.unwrap().0, new_state.last_move.unwrap().1), score);
			}
			i += 1;
            if score > current {
                current = score;
                best_move = new_state.last_move;
                alpha = score.max(alpha);
                if alpha >= beta {
                    break;
                }
				tmp_beta = alpha + 1;
            }
        }
        state.selected_move = best_move;
        current
    }
}