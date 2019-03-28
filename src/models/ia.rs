use crate::models::gameboard::*;
use std::collections::HashSet;
use std::cmp::min;
use std::cmp::max;
use std::process::exit;
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

pub fn evale_one_line(mut line: u64, stone: u8) -> isize {
	let mut value = 0;
	let mut i: isize = 0;
	let mut j: isize = 0;

	while i < 64 {
		match (line & 0b1111_1111_1111) as u16 {
			0b0000_0000_0000 => {  // ALIGN NULL
					j = 10;
					i += 10;
			},
			0b0110_1010_1000 | 0b0010_1010_1001 => {  // ALIGN 4/
					value -= 10000;
					j = 10;
					i += 10;
			},
			0b0010_1010_1010 | 0b1010_1010_1000 | 0b1010_1010_1001 | 0b0110_1010_1010 => {  // ALIGN 5
					value -= 10000000;
					j = 10;
					i += 10;
			},
			// is_open if (is_open & 0b0011_1111_1100) != 0 => {

			// }
			0b0010_1010_1000 => {  // ALIGN 4
					value -= 100000;
					j = 10;
					i += 10;
			},
			0b0010_0010_1000 | 0b0010_1000_1000 | 0b0000_1010_1000 => { // ALIGN 3
					value -= 1000;
					j = 10;
					i += 10;
			},
			0b0010_0010_1001 | 0b0010_1000_1001 | 0b0000_1010_1001 => { // ALIGN 3/
					value -= 100;
					j = 10;
					i += 10;
			},
			0b0110_0010_1000 | 0b0110_1000_1000 | 0b0100_1010_1000 => { // ALIGN /3
					value -= 100;
					j = 10;
					i += 10;
			},
			0b0000_1010_0000 => { //ALIGN 2
					value -= 100;
					j = 10;
					i += 10;
			},

			0b0001_0101_0101 | 0b0101_0101_0100 | 0b0101_0101_0110 | 0b1001_0101_0101 => {  // ALIGN 5
					value += 10000000;
					j = 10;
					i += 10;
			},
			0b1001_0101_0100 | 0b0001_0101_0110 => {  // ALIGN 4/
					value += 10000;
					j = 10;
					i += 10;
			},
			0b0001_0101_0100 => {  // ALIGN 4
					value += 100000;
					j = 10;
					i += 10;
			},
			0b0001_0100_0100 | 0b0001_0001_0100 | 0b0000_0101_0100 => { // ALIGN 3
					value += 1000;
					j = 10;
					i += 10;
			},
			0b0001_0100_0110 | 0b0001_0001_0110 | 0b0000_0101_0110 => { // ALIGN 3/
					value += 100;
					j = 10;
					i += 10;
			},
			0b1001_0100_0100 | 0b1001_0001_0100 | 0b1000_0101_0100 => { // ALIGN /3
					value += 100;
					j = 10;
					i += 10;
			},

			0b0000_0101_0000 => { //ALIGN 2
					value += 100;
					j = 10;
					i += 10;
			},
			_ => {
					j = 2;
					i += 2;
			},
		}
		line >>= j;
	}
	value
}

impl IA {
    pub fn is_victory(&self) -> bool {
        false
    }

    pub fn eval(&self, state: &Gameboard, stone: u8) -> isize {
		// println!("\n\n______ EVAL _______");
		// printboard!(&state.cells);

		let mut value: isize = 0;
		if state.black_captures >= 10 {
			value = 10000000;
		} else if state.white_captures >= 10 {
			value = -10000000;
		}
		else {
			let mut all: Vec<u64> = (0..SIZE).map(|y| line_horizontal!(state.cells, 0, SIZE - 1, y as usize)).collect();
			let all_verti: Vec<u64> = (0..SIZE).map(|x| line_vertical!(state.cells[x as usize], 0 , SIZE -1)).collect();
			let all_diag_d: Vec<u64> = (0..SIZE).map(|x| down_diago!(state.cells, x, 0, x as usize, 0)).collect();
			let all_diag_d2: Vec<u64> = (1..SIZE).map(|y| down_diago_orig!(state.cells, SIZE - 1, 0, SIZE - 1, y as usize, 0, SIZE - 1)).collect();
			let all_diag_u: Vec<u64> = (0..SIZE).map(|x| up_diago_orig!(state.cells, x as usize, 0, SIZE -1, 0, 0, SIZE - 1)).collect();
			let all_diag_u2: Vec<u64> = (1..SIZE).map(|y| up_diago_orig!(state.cells, 0, 0, SIZE -1, y as usize, 0, SIZE - 1)).collect();

			all.extend(all_verti);
			all.extend(all_diag_d);
			all.extend(all_diag_d2);
			all.extend(all_diag_u);
			all.extend(all_diag_u2);
			all.retain(|&elem| elem != 0);


			for e in all {
				value += evale_one_line(e, stone);
			}
		}
		if stone == WHITE {
			-value
		} else {
			value
		}
	}
}

impl IA {
	/// si alpha < current < beta, alors current est la valeur minimax
    /// si current <= alpha, alors la vraie valeur minimax m vérifie : m <= current <= alpha
    /// si beta <= current alors la vraie valeur minimax m vérifie : beta <= current <= m
    pub fn negascout(&self, state: &mut Gameboard, transposition_table: &mut HashSet<Gameboard>, stone: u8, depth: u8, mut alpha: isize, beta: isize) -> isize {
        // if depth % 2 == 0 && transposition_table.contains(state) {
		// 	state.value = transposition_table.get(state).unwrap().value;			
		// 	return state.value
		// }
		if depth == 0 || state.is_finish() {
			state.value = self.eval(state, stone);
			// if depth % 2 == 0 {
			// 	transposition_table.insert(state.clone());
			// }
            return state.value;
        }
        let mut best_move: Option<(usize, usize)> = None;
        let mut current = (std::i64::MIN + 1) as isize;
        let mut last_move = (0, 0);
		let mut tmp_beta = beta;
		let mut i = 0;
        loop {
            state.next_move(last_move.0, last_move.1);
            let new_move = match state.selected_move {
                Some(new_move) => new_move,
                None => break,
            };
            let mut new_state = state.clone();
            new_state.make_move(new_move.0, new_move.1, stone);
            let mut score = -self.negascout(&mut new_state, transposition_table, opposite_stone!(stone), depth - 1, -tmp_beta, -alpha);
            if score > alpha && score < beta && i > 0 && depth > 1 {
                score = -self.negascout(&mut new_state, transposition_table, opposite_stone!(stone), depth - 1, -beta, -alpha);
            }
			i += 1;
            if score > current {
                current = score;
                best_move = Some(new_move);
                alpha = score.max(alpha);
                if alpha >= beta {
                    break;
                }
				tmp_beta = alpha + 1;
            }
            last_move = (new_move.0 + 1, new_move.1);
        }
        state.selected_move = best_move;
        current
    }

    pub fn alphabeta(&self, state: &mut Gameboard, transposition_table: &mut HashSet<Gameboard>, stone: u8, depth: u8, mut alpha: isize, beta: isize) -> isize {
        if depth % 2 == 0 && transposition_table.contains(state) {
			state.value = transposition_table.get(state).unwrap().value;
			return state.value
		}
		if depth == 0 || state.is_finish() {
			state.value = self.eval(state, stone);
			if depth % 2 == 0 {
				transposition_table.insert(state.clone());
			}
            return state.value;
        }
        let mut best_move: Option<(usize, usize)> = None;
        let mut current = isize::from(std::i16::MIN);
        let mut last_move = (0, 0);
        loop {
            state.next_move(last_move.0, last_move.1);
            let new_move = match state.selected_move {
                Some(new_move) => new_move,
                None => break,
            };
            let mut new_state = state.clone();
            new_state.make_move(new_move.0, new_move.1, stone);
            let mut score = -self.alphabeta(&mut new_state, transposition_table, opposite_stone!(stone), depth - 1, -beta, -alpha);
            if score > current {
                current = score;
                best_move = Some(new_move);
                alpha = score.max(alpha);
                if alpha >= beta {
                    break;
                }
            }
            last_move = (new_move.0 + 1, new_move.1);
        }
        state.selected_move = best_move;
        alpha
    }
}