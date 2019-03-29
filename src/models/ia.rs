use crate::models::gameboard::*;
use crate::eval::*;
use std::collections::HashSet;
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
	/// si alpha < current < beta, alors current est la valeur minimax
    /// si current <= alpha, alors la vraie valeur minimax m vérifie : m <= current <= alpha
    /// si beta <= current alors la vraie valeur minimax m vérifie : beta <= current <= m

    pub fn negascout(&self, state: &mut Gameboard, transposition_table: &mut HashSet<Gameboard>, stone: u8, depth: u8, mut alpha: isize, beta: isize, map_board_values: &mut HashMap<[u64; SIZE], isize>) -> isize {
        // if transposition_table.contains(state) {
		// 	state.value = transposition_table.get(state).unwrap().value;			
		// 	return state.value
		// }
		if depth == 0 || state.is_finish() {
			state.value = eval(state, stone, depth, map_board_values);
			// transposition_table.insert(state.clone());
            return state.value;
        }
		//   if depth % 2 == 0 && transposition_table.contains(state) {
		// 	state.value = transposition_table.get(state).unwrap().value;			
		// 	return state.value
		// }
        let mut best_move: Option<(usize, usize)> = None;
        let mut current = (std::i64::MIN + 1) as isize;
        let mut last_move = (0, 0);
        loop {
            state.next_move(last_move.0, last_move.1);
            let new_move = match state.selected_move {
                Some(new_move) => new_move,
                None => break,
            };
            let mut new_state = state.clone();
            new_state.make_move(new_move.0, new_move.1, stone);
            let mut score = -self.negascout(&mut new_state, transposition_table, opposite_stone!(stone), depth - 1, -(alpha + 1), -alpha, map_board_values);
            if score > alpha && score < beta {
                score = -self.negascout(&mut new_state, transposition_table, opposite_stone!(stone), depth - 1, -beta, -alpha, map_board_values);
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
		current
    }

    // pub fn alphabeta(&self, state: &mut Gameboard, transposition_table: &mut HashSet<Gameboard>, stone: u8, depth: u8, mut alpha: isize, beta: isize) -> isize {
    //     if depth % 2 == 0 && transposition_table.contains(state) {
	// 		state.value = transposition_table.get(state).unwrap().value;
	// 		return state.value
	// 	}
	// 	if depth == 0 || state.is_finish() {
	// 		state.value = self.eval(state, stone);
	// 		if depth % 2 == 0 {
	// 			transposition_table.insert(state.clone());
	// 		}
    //         return state.value;
    //     }
    //     let mut best_move: Option<(usize, usize)> = None;
    //     let mut current = isize::from(std::i16::MIN);
    //     let mut last_move = (0, 0);
    //     loop {
    //         state.next_move(last_move.0, last_move.1);
    //         let new_move = match state.selected_move {
    //             Some(new_move) => new_move,
    //             None => break,
    //         };
    //         let mut new_state = state.clone();
    //         new_state.make_move(new_move.0, new_move.1, stone);
    //         let score = -self.alphabeta(&mut new_state, transposition_table, opposite_stone!(stone), depth - 1, -beta, -alpha);
    //         if score >= current {
    //             current = score;
    //             best_move = Some(new_move);
    //             alpha = score.max(alpha);
    //             if alpha >= beta {
    //                 break;
    //             }
    //         }
    //         last_move = (new_move.0 + 1, new_move.1);
    //     }
    //     state.selected_move = best_move;
    //     alpha
    // }
}


// pub fn evale_one_line_old(mut line: u64) -> isize {
// 	let mut value = 0;
// 	// let mut i: isize = 0;
// 	let mut j: isize;

// 	// println!("EVAL one line: {:#066b}", line);

// 	while line != 0 {
// 		dbg_line((line & 0b1111_1111_1111) as u16);
// 		// println!("eval: {:#016b} ", line & 0b1111_1111_1111);
// 		match (line & 0b1111_1111_1111) as u16 {
// 			0b00_00_00_00_00_00 => {  // ALIGN NULL
// 					j = 10;
// 			},
// 			0b01_10_10_10_10_00 | 0b00_10_10_10_10_01 => {  // ALIGN 4/
// 		dbg_line((line & 0b1111_1111_1111) as u16);
// 					println!("value -= 10000");

// 					value -= 10000;
// 					j = 10;
// 			},
// 			0b00_10_10_10_10_10 | 0b10_10_10_10_10_00 | 0b10_10_10_10_10_01 | 0b01_10_10_10_10_10 => {  // ALIGN 5
// 		dbg_line((line & 0b1111_1111_1111) as u16);
// 					println!("value -= 10000000");

// 					value -= 10000000;
// 					j = 10;
// 			},
// 			0b00_10_10_10_10_00 => {  // ALIGN 4
// 		dbg_line((line & 0b1111_1111_1111) as u16);
// 					println!("value -= 100000");

// 					value -= 100000;
// 					j = 10;
// 			},
// 			0b00_10_00_10_10_00 | 0b00_10_10_00_10_00 | 0b00_00_10_10_10_00 => { // ALIGN 3
// 		dbg_line((line & 0b1111_1111_1111) as u16);
// 					println!("value -= 1000");

// 					value -= 1000;
// 					j = 10;
// 			},
// 			0b00_10_00_10_10_01 | 0b00_10_10_00_10_01 | 0b00_00_10_10_10_01 => { // ALIGN 3/
// 		dbg_line((line & 0b1111_1111_1111) as u16);
// 					println!("value -= 100");

// 					value -= 100;
// 					j = 10;
// 			},
// 			0b01_10_00_10_10_00 | 0b01_10_10_00_10_00 | 0b0100_10_10_10_00 => { // ALIGN /3
// 		dbg_line((line & 0b1111_1111_1111) as u16);
// 					println!("value -= 100");

// 					value -= 100;
// 					j = 10;
// 			},
// 			0b00_00_10_10_00_00 | 0b10_10_10_10_00_00 => { //ALIGN 2
// 		dbg_line((line & 0b1111_1111_1111) as u16);
// 					println!("value -= 100");

// 					value -= 100;
// 					j = 10;
// 			},

// 			0b00_01_01_01_01_01 | 0b01_01_01_01_01_00 | 0b01_01_01_01_01_10 | 0b10_01_01_01_01_01 => {  // ALIGN 5
// 		dbg_line((line & 0b1111_1111_1111) as u16);
// 					println!("value += 10000000");

// 					value += 10000000;
// 					j = 10;
// 			},
// 			0b10_01_01_01_01_00 | 0b00_01_01_01_01_10 => {  // ALIGN 4/
// 		dbg_line((line & 0b1111_1111_1111) as u16);
// 					println!("value += 10000");

// 					value += 10000;
// 					j = 10;
// 			},
// 			0b00_01_01_01_01_00 => {  // ALIGN 4
// 		dbg_line((line & 0b1111_1111_1111) as u16);
// 					println!("value += 100000");

// 					value += 100000;
// 					j = 10;
// 			},
// 			0b00_01_01_00_01_00 | 0b00_01_00_01_01_00 | 0b00_00_01_01_01_00 => { // ALIGN 3
// 		dbg_line((line & 0b1111_1111_1111) as u16);
// 					println!("value += 1000");

// 					value += 1000;
// 					j = 10;
// 			},
// 			0b00_01_01_00_01_10 | 0b00_01_00_01_01_10 | 0b00_00_01_01_01_10 => { // ALIGN 3/
// 		dbg_line((line & 0b1111_1111_1111) as u16);
// 					println!("value += 100");

// 					value += 100;
// 					j = 10;
// 			},
// 			0b10_01_01_00_01_00 | 0b10_01_00_01_01_00 | 0b10_00_01_01_01_00 => { // ALIGN /3
// 		dbg_line((line & 0b1111_1111_1111) as u16);
// 					println!("value += 100");

// 					value += 100;
// 					j = 10;
// 			},

// 			0b00_00_01_01_00_00 => { //ALIGN 2
// 		dbg_line((line & 0b1111_1111_1111) as u16);
// 					println!("value += 100");

// 					value += 100;
// 					j = 10;
// 			},
// 			_ => {
// 					j = 2;
// 			},
// 		}
// 		line >>= j;
// 	}
// 	if value != 0 {
// 		println!("value: {}", value);
// 	}
// 	value
// }
