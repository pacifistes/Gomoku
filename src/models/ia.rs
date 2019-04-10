use crate::controllers::game::print_all_values;
use crate::models::gameboard::*;
use crate::eval::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::cmp::min;
use std::cmp::max;
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct IA {
    pub depth: u8,
	pub counter: usize,
	pub g: isize,
}

impl IA {
    pub fn new(depth: u8) -> IA {
        IA {
            depth,
			counter: 0,
			g: 0,
        }
    }
}

impl IA {
	pub fn expand(&self, state: &Gameboard, stone: u8, depth: u8, player_stone: u8) -> Vec<Gameboard> {
		let possible_moves: Vec<(usize, usize)> = state.expand();
		// if depth == 2 {
            // println!("{}", possible_moves.len());
        // }
		let mut possible_boards: Vec<Gameboard> = possible_moves.iter().filter_map(|new_move| {
			let mut new_state = state.clone();
			new_state.result = None;
			if (new_state.make_move(new_move.0, new_move.1, stone)) {
				Some(new_state)
			}
			else {
				None
			}
		}).collect();

		let mut sort_by_min = true;
		if player_stone == BLACK {
			sort_by_min = !sort_by_min;
		}
		if stone != player_stone {
			sort_by_min = !sort_by_min;
		}
		if (!sort_by_min) {
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

	pub fn negascout(&mut self, state: &mut Gameboard, stone: u8, depth: u8, mut alpha: isize, beta: isize, map_board_values: &mut HashMap<[u64; SIZE], isize>, all_values: &mut HashMap<(usize, usize), isize>,  player_stone: u8) -> isize {
		if depth == 0 || state.is_finish() {
			let mut score = state.value;
			score *= depth as isize + 1;
			if player_stone == BLACK {
				score = -score;
			}
			if stone == player_stone {
	            // println!("{}, {}", self.counter, score);
				return score;
			} else {
	            // println!("{}, {}",self.counter,  -score);
				return -score;
			}
        }
        let mut best_move: Option<(usize, usize)> = None;
        let mut current = (std::i64::MIN + 1) as isize;
		let mut tmp_beta = beta;
		let mut i = 0;
		let possible_states: Vec<Gameboard> = self.expand(state, stone, depth, player_stone);
        for mut new_state in possible_states {
			self.counter += 1;
            let mut score = -self.negascout(&mut new_state, opposite_stone!(stone), depth - 1, -tmp_beta, -alpha, map_board_values, all_values, player_stone);
            if score > alpha && score < beta && i > 0 && depth > 1 {
				self.counter += 1;
                score = -self.negascout(&mut new_state, opposite_stone!(stone), depth - 1, -beta, -alpha, map_board_values, all_values, player_stone);
            }
			if depth == self.depth {
				all_values.insert((new_state.last_move.unwrap().0, new_state.last_move.unwrap().1), score);
			}
			i += 1;
			println!("depth = {}, alpha = {}, beta, = {}, value = {}", depth, alpha, beta, score);
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
		println!("final depth = {}, alpha = {}, beta, = {}, value = {}", depth, alpha, beta, current);

        state.selected_move = best_move;
        current
    }

	pub fn alphabeta(&mut self, state: &mut Gameboard, stone: u8, depth: u8, mut alpha: isize, beta: isize, map_board_values: &mut HashMap<[u64; SIZE], isize>, all_values: &mut HashMap<(usize, usize), isize>,  player_stone: u8) -> isize {
		if depth == 0 || state.is_finish() {
			let mut score = state.value;
			score *= depth as isize + 1;
			if player_stone == BLACK {
				score = -score;
			}
			if stone == player_stone {
	            // println!("{}, {}", self.counter, score);
				return score;
			} else {
	            // println!("{}, {}",self.counter,  -score);
				return -score;
			}
        }
        let mut best_move: Option<(usize, usize)> = None;
        let mut current = (std::i64::MIN + 1) as isize;
		let possible_states: Vec<Gameboard> = self.expand(state, stone, depth, player_stone);
        for mut new_state in possible_states {
			self.counter += 1;
			let score = -self.alphabeta(&mut new_state, opposite_stone!(stone), depth - 1, -beta, -alpha, map_board_values, all_values, player_stone);
			if depth == self.depth {
				all_values.insert((new_state.last_move.unwrap().0, new_state.last_move.unwrap().1), score);
			}
			// if (score == 9910 || score == 10020) {
			// 	println!("score before = {}", state.value);
			// }
            if score > current {
                current = score;
                best_move = new_state.last_move;
                alpha = score.max(alpha);
                if alpha >= beta {
                    break;
                }
            }
        }
        state.selected_move = best_move;
        current
    }

	pub fn alphabeta_tt(&mut self, state: &mut Gameboard, transposition_table: &mut HashSet<Gameboard>, stone: u8, depth: u8, mut alpha: isize, mut beta: isize, map_board_values: &mut HashMap<[u64; SIZE], isize>, all_values: &mut HashMap<(usize, usize), isize>,  player_stone: u8) -> isize {
        if transposition_table.contains(state) {
			*state = transposition_table.get(state).unwrap().clone();
            if state.lowerbound >= beta {
                return state.lowerbound;
            }
            if state.upperbound <= alpha {
                return state.upperbound;
            }
            alpha = max(alpha, state.lowerbound);
            beta = min(beta, state.upperbound);
        }
		if depth == 0 || state.is_finish() {
			let mut score = state.value;
			score *= depth as isize + 1;
			if player_stone == BLACK {
				score = -score;
			}
			if stone == player_stone {
	            // println!("{}, {}", self.counter, score);
				return score;
			} else {
	            // println!("{}, {}",self.counter,  -score);
				return -score;
			}
		}
	
	
		let mut best_move: Option<(usize, usize)> = None;
        let mut current = (std::i64::MIN + 1) as isize;
		let possible_states: Vec<Gameboard> = self.expand(state, stone, depth, player_stone);
        for mut new_state in possible_states {
			self.counter += 1;
            let score = -self.alphabeta_tt(&mut new_state, transposition_table, opposite_stone!(stone), depth - 1, -beta, -alpha, map_board_values, all_values, player_stone);
			if depth == self.depth {
				all_values.insert((new_state.last_move.unwrap().0, new_state.last_move.unwrap().1), score);
			}
			if (depth == self.depth - 1) {
				println!("value = {}", score);
			}
            if score > current {
                current = score;
                best_move = new_state.last_move;
                alpha = score.max(alpha);
                if alpha >= beta {
                    break;
                }
            }
        }
		if (depth == self.depth - 1) {
			println!("beta = {}, value = {}", beta, current);
		}
		state.selected_move = best_move;
        if current <= alpha {
            state.upperbound = current;
			transposition_table.insert(state.clone());
        }
        if current >= beta {
            state.lowerbound = current;
			transposition_table.insert(state.clone());
        }
        return current;
    }

	pub fn negascout_tt(&mut self, state: &mut Gameboard, transposition_table: &mut HashSet<Gameboard>, stone: u8, depth: u8, mut alpha: isize, mut beta: isize, map_board_values: &mut HashMap<[u64; SIZE], isize>, all_values: &mut HashMap<(usize, usize), isize>,  player_stone: u8) -> isize {
        if transposition_table.contains(state) {
			*state = transposition_table.get(state).unwrap().clone();
            if state.lowerbound >= beta {
                return state.lowerbound;
            }
            if state.upperbound < alpha {
                return state.upperbound;
            }
            alpha = max(alpha, state.lowerbound);
            beta = min(beta, state.upperbound);
        }
		if depth == 0 || state.is_finish() {
			let mut score = state.value;
			score *= depth as isize + 1;
			if player_stone == BLACK {
				score = -score;
			}
			if stone == player_stone {
	            // println!("{}, {}", self.counter, score);
				return score;
			} else {
	            // println!("{}, {}",self.counter,  -score);
				return -score;
			}
		}

        let mut best_move: Option<(usize, usize)> = None;
        let mut current = (std::i64::MIN + 1) as isize;
		let mut tmp_beta = beta;
		let mut i = 0;
		let possible_states: Vec<Gameboard> = self.expand(state, stone, depth, player_stone);
        for mut new_state in possible_states {
			self.counter += 1;
            let mut score = -self.negascout_tt(&mut new_state, transposition_table, opposite_stone!(stone), depth - 1, -tmp_beta, -alpha, map_board_values, all_values, player_stone);
            if score > alpha && score < beta && i > 0 && depth > 1 {
				self.counter += 1;
                score = -self.negascout_tt(&mut new_state, transposition_table, opposite_stone!(stone), depth - 1, -beta, -alpha, map_board_values, all_values, player_stone);
            }
			if depth == self.depth {
				all_values.insert((new_state.last_move.unwrap().0, new_state.last_move.unwrap().1), score);
			}
            if score > current {
                current = score;
                best_move = new_state.last_move;
                alpha = score.max(alpha);
                if alpha >= beta {
                    break;
                }
            }
        }
		state.selected_move = best_move;
        if current < alpha {
            state.upperbound = current;
			transposition_table.insert(state.clone());
        }
        if current >= beta {
            state.lowerbound = current;
			transposition_table.insert(state.clone());
        }
        return current;
    }

























	pub fn alphabeta_with_memory(&mut self, state: &mut Gameboard, transposition_table: &mut HashSet<Gameboard>, stone: u8, depth: u8, mut alpha: isize, mut beta: isize, map_board_values: &mut HashMap<[u64; SIZE], isize>, all_values: &mut HashMap<(usize, usize), isize>,  player_stone: u8) -> isize {
        if /*self.depth != depth &&*/ transposition_table.contains(state) {
			*state = transposition_table.get(state).unwrap().clone();
			println!("lower = {}, beta = {}, upper = {}, alpha = {}", state.lowerbound, beta, state.upperbound, alpha);
            if state.lowerbound >= beta {
                return state.lowerbound;
            }
            if state.upperbound <= alpha {
                return state.upperbound;
            }
            alpha = max(alpha, state.lowerbound);
            beta = min(beta, state.upperbound);
        }
		if depth == 0 || state.is_finish() {
			let mut score = state.value;
			score *= depth as isize + 1;
			if player_stone == BLACK {
				score = -score;
			}
			if stone == player_stone {
	            // println!("{}, {}", self.counter, score);
				return score;
			} else {
	            // println!("{}, {}",self.counter,  -score);
				return -score;
			}
		}
	
	
		let mut best_move: Option<(usize, usize)> = None;
        let mut current = (std::i64::MIN + 1) as isize;
        let mut tmp_alpha = alpha;
		let possible_states: Vec<Gameboard> = self.expand(state, stone, depth, player_stone);
        for mut new_state in possible_states {
			self.counter += 1;
            let score = -self.alphabeta_with_memory(&mut new_state, transposition_table, opposite_stone!(stone), depth - 1, -beta, -tmp_alpha, map_board_values, all_values, player_stone);
			if depth == self.depth {
				all_values.insert((new_state.last_move.unwrap().0, new_state.last_move.unwrap().1), score);
			}
			println!("depth = {}, tmp_alpha = {}, alpha = {}, beta, = {}, value = {}", depth, tmp_alpha, alpha, beta, score);
            if score >= current {
                current = score;
                best_move = new_state.last_move;
                tmp_alpha = score.max(tmp_alpha);
                if tmp_alpha >= beta {
                    break;
                }
            }
        }
		println!("final depth = {}, tmp_alpha = {}, alpha = {}, beta, = {}, value = {}", depth, tmp_alpha, alpha, beta, current);
		state.selected_move = best_move;
		let mut tmp_state = state.clone();
        if current <= alpha {
            tmp_state.upperbound = current;
			transposition_table.insert(tmp_state.clone());
        }
        if current >= beta {
            tmp_state.lowerbound = current;
			transposition_table.insert(tmp_state.clone());
        }
        return current;
    }

	pub fn mtdf(&mut self, state: &mut Gameboard, stone: u8, depth: u8, map_board_values: &mut HashMap<[u64; SIZE], isize>, all_values: &mut HashMap<(usize, usize), isize>,  player_stone: u8) { //On utilise donc en général comme valeur de f la valeur retourné par l’algorithme lors d’une itération précédente
		let mut upperbound = std::i64::MAX as isize;
		let mut lowerbound = std::i64::MIN as isize;
		let mut transposition_table: HashSet<Gameboard> = HashSet::new();
		
		let mut best_move: Option<(usize, usize)> = None;
		while lowerbound < upperbound {
			let beta: isize = match lowerbound {
				elem if self.g == elem	=> self.g + 1,
				_ 					=> self.g,
			};
			println!("before alpha {} | beta = {} | g = {}", beta - 1, beta, self.g);
			self.g = self.alphabeta_with_memory(state, &mut transposition_table, stone, depth, beta - 1, beta, map_board_values, all_values, player_stone);
			// self.g = self.alphabeta_tt(state, &mut transposition_table, stone, depth, beta - 1, beta, map_board_values, all_values, player_stone);
			// print_all_values(&state.cells, &all_values);
			println!("before {} | {} | beta = {} | g = {}", lowerbound, upperbound, beta, self.g);
			if self.g < beta {
				upperbound = self.g;
			}
			else {
				// best_move = state.selected_move;
				lowerbound = self.g;
			}
			println!("after {} | {}", lowerbound, upperbound);
		}
		// state.selected_move = best_move;
	}
}








