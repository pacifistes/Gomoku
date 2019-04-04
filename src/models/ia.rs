use crate::models::gameboard::*;
use crate::eval::*;

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
		possible_boards.sort_by(|board, other| board.value.cmp(&other.value));
		// let len = possible_boards.len();
		// let nbr_item = len.min(4);
		// let nbr_item = len.min(4 + self.depth as usize);
		// possible_boards = possible_boards[0..nbr_item].to_vec();
		possible_boards
	}

	pub fn negascout(&self, state: &mut Gameboard, stone: u8, depth: u8, mut alpha: isize, beta: isize) -> isize {
		if depth == 0 || state.is_finish() {
			if stone == BLACK {
				return -state.value;
			} else {
				return state.value;
			}
        }
        let mut best_move: Option<(usize, usize)> = None;
        let mut current = (std::i64::MIN + 1) as isize;
		let mut tmp_beta = beta;
		let mut i = 0;
		let possible_states: Vec<Gameboard> = self.expand(state, stone, depth);
        for mut new_state in possible_states {
            let mut score = -self.negascout(&mut new_state, opposite_stone!(stone), depth - 1, -tmp_beta, -alpha);
            if score > alpha && score < beta && i > 0 && depth > 1 {
                score = -self.negascout(&mut new_state, opposite_stone!(stone), depth - 1, -beta, -alpha);
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