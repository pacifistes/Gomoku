use crate::models::gameboard::*;
use std::collections::HashSet;
use std::cmp::min;
use std::cmp::max;

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

pub fn evale_one_line(mut line: u64) -> isize {
	let mut value = 0;
	let mut i: isize = 0;
	let mut j: isize = 0;

	while i < 64 {
		match (line & 0b1111_1111_1111) as u16 {
			0b00_00_00_00_00_00 => {  // ALIGN NULL
					j = 10;
					i += 10;
			},
			0b01_10_10_10_10_00 | 0b00_10_10_10_10_01 => {  // ALIGN 4/
					value -= 10000;
					j = 10;
					i += 10;
			},
			0b00_10_10_10_10_10 | 0b10_10_10_10_10_00 | 0b10_10_10_10_10_01 | 0b01_10_10_10_10_10 => {  // ALIGN 5
					value -= 10000000;
					j = 10;
					i += 10;
			},
			0b00_10_10_10_10_00 => {  // ALIGN 4
					value -= 100000;
					j = 10;
					i += 10;
			},
			0b00_10_00_10_10_00 | 0b00_10_10_00_10_00 | 0b00_00_10_10_10_00 => { // ALIGN 3
					value -= 1000;
					j = 10;
					i += 10;
			},
			0b00_10_00_10_10_01 | 0b00_10_10_00_10_01 | 0b00_00_10_10_10_01 => { // ALIGN 3/
					value -= 100;
					j = 10;
					i += 10;
			},
			0b01_10_00_10_10_00 | 0b01_10_10_00_10_00 | 0b0100_10_10_10_00 => { // ALIGN /3
					value -= 100;
					j = 10;
					i += 10;
			},
			0b00_00_10_10_00_00 => { //ALIGN 2
					value -= 100;
					j = 10;
					i += 10;
			},

			0b00_01_01_01_01_01 | 0b01_01_01_01_01_00 | 0b01_01_01_01_01_10 | 0b10_01_01_01_01_01 => {  // ALIGN 5
					value += 10000000;
					j = 10;
					i += 10;
			},
			0b10_01_01_01_01_00 | 0b00_01_01_01_01_10 => {  // ALIGN 4/
					value += 10000;
					j = 10;
					i += 10;
			},
			0b00_01_01_01_01_00 => {  // ALIGN 4
					value += 100000;
					j = 10;
					i += 10;
			},
			0b00_01_01_00_01_00 | 0b00_01_00_01_01_00 | 0b00_00_01_01_01_00 => { // ALIGN 3
					value += 1000;
					j = 10;
					i += 10;
			},
			0b00_01_01_00_01_10 | 0b00_01_00_01_01_10 | 0b00_00_01_01_01_10 => { // ALIGN 3/
					value += 100;
					j = 10;
					i += 10;
			},
			0b10_01_01_00_01_00 | 0b10_01_00_01_01_00 | 0b10_00_01_01_01_00 => { // ALIGN /3
					value += 100;
					j = 10;
					i += 10;
			},

			0b00_00_01_01_00_00 => { //ALIGN 2
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
    pub fn eval(&self, state: &Gameboard, stone: u8) -> isize {

		println!("\n\n______ EVAL _______");
		printboard!(&state.cells);
		if (state.black_captures >= 10 && stone == WHITE) || (state.white_captures >= 10 && stone == BLACK){
			-10000000
		} else if state.white_captures >= 10  && stone == WHITE || state.black_captures >= 10  && stone == BLACK {
			10000000
		} else {
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

			let value: isize = all.iter().map(|&e| evale_one_line(e)).sum();
			// if stone == WHITE {
				// -value
			// } else {
				value
			// }
		}
	}
}

impl IA {
	/// si alpha < current < beta, alors current est la valeur minimax
    /// si current <= alpha, alors la vraie valeur minimax m vérifie : m <= current <= alpha
    /// si beta <= current alors la vraie valeur minimax m vérifie : beta <= current <= m
    pub fn negascout(&self, state: &mut Gameboard, transposition_table: &mut HashSet<Gameboard>, stone: u8, depth: u8, mut alpha: isize, beta: isize) -> isize {
        if depth % 2 == 0 && transposition_table.contains(state) {
			return state.value
		}
		if depth == 0 || state.is_finish() {
			state.value = self.eval(state, stone);
			println!("VALUE: {}", state.value);
			if depth % 2 == 0 {
				transposition_table.insert(state.clone());
			}
            return state.value;
        }

        let mut best_move: Option<(usize, usize)> = None;
        let mut current = (std::i64::MIN +1) as isize;
        let mut last_move = (0, 0);
        loop {
            state.next_move(last_move.0, last_move.1);
            let new_move = match state.selected_move {
                Some(new_move) => new_move,
                None => break,
            };
            let mut new_state = state.clone();
            new_state.make_move(new_move.0, new_move.1, stone);
            let mut score = -self.negascout(&mut new_state, transposition_table, opposite_stone!(stone), depth - 1, -(alpha + 1), -alpha);
            if score > alpha && score < beta {
                score = -self.negascout(&mut new_state, transposition_table, opposite_stone!(stone), depth - 1, -beta, -alpha);
            }
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

    pub fn alphabeta(&self, state: &mut Gameboard, transposition_table: &mut HashSet<Gameboard>, stone: u8, depth: u8, mut alpha: isize, beta: isize) -> isize {
        if depth % 2 == 0 && transposition_table.contains(state) {
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
// <<<<<<< HEAD
//             let score = -self.alphabeta(&mut new_state, opposite_stone!(stone), depth - 1, -beta, -alpha);
// =======
            let mut score = -self.alphabeta(&mut new_state, transposition_table, opposite_stone!(stone), depth - 1, -beta, -alpha);
// >>>>>>> master
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