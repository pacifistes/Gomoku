use crate::models::game::GameResult;
use crate::eval::evale_one_line;
use std::hash::{Hash, Hasher};
use std::cmp::Ordering;
use std::iter::Sum;

/// Size of game board.
pub const SIZE: usize = 19;

pub const NOPE: u8 = 0b00;
pub const BLACK: u8 = 0b01;
pub const WHITE: u8 = 0b10;

pub const WHITE_CAPTURE: u8 = WHITE | BLACK << 2 | BLACK << 4 | WHITE << 6;
pub const BLACK_CAPTURE: u8 = BLACK | WHITE << 2 | WHITE << 4 | BLACK << 6;

pub const BLACK_5_ALIGN: u16 = BLACK as u16 | (BLACK as u16) << 2 | (BLACK as u16) << 4 | (BLACK as u16) << 6 | (BLACK as u16) << 8;
pub const WHITE_5_ALIGN: u16 = WHITE as u16 | (WHITE as u16) << 2 | (WHITE as u16) << 4 | (WHITE as u16) << 6 | (WHITE as u16) << 8;
pub const BLACK_TREES: [u16; 4] = [
	NOPE as u16 | (BLACK as u16) << 2 | (BLACK as u16) << 4 | (BLACK as u16) << 6 | (NOPE as u16) << 8 | (NOPE as u16) << 10,
	NOPE as u16 | (BLACK as u16) << 2 | (BLACK as u16) << 4 | (NOPE as u16) << 6 | (BLACK as u16) << 8 | (NOPE as u16) << 10,
	NOPE as u16 | (BLACK as u16) << 2 | (NOPE as u16) << 4 | (BLACK as u16) << 6 | (BLACK as u16) << 8 | (NOPE as u16) << 10,
	NOPE as u16 | (NOPE as u16) << 2 | (BLACK as u16) << 4 | (BLACK as u16) << 6 | (BLACK as u16) << 8 | (NOPE as u16) << 10,
];
pub const WHITE_TREES: [u16; 4] = [
	NOPE as u16 | (WHITE as u16) << 2 | (WHITE as u16) << 4 | (WHITE as u16) << 6 | (NOPE as u16) << 8 | (NOPE as u16) << 10,
	NOPE as u16 | (WHITE as u16) << 2 | (WHITE as u16) << 4 | (NOPE as u16) << 6 | (WHITE as u16) << 8 | (NOPE as u16) << 10,
	NOPE as u16 | (WHITE as u16) << 2 | (NOPE as u16) << 4 | (WHITE as u16) << 6 | (WHITE as u16) << 8 | (NOPE as u16) << 10,
	NOPE as u16 | (NOPE as u16) << 2 | (WHITE as u16) << 4 | (WHITE as u16) << 6 | (WHITE as u16) << 8 | (NOPE as u16) << 10,
];

#[derive(Debug, Eq, Clone, Copy)]
pub struct Line {
	pub score: isize,
	pub representation: u64,
}

#[derive(Debug, Eq, Clone)]
pub struct Gameboard {
    pub cells: [u64; SIZE],
	pub possible_moves: [u32; SIZE],
	pub lines: [[[Line; 4]; SIZE]; SIZE],
    pub selected_move: Option<(usize, usize)>,
    pub last_move: Option<(usize, usize)>,
	pub black_captures: u8,
	pub white_captures: u8,
	pub value: isize,
	pub result: Option<GameResult>,
}


impl Gameboard {
	pub fn new() -> Gameboard {
		let score = 0;
		let representation = 0;
		Gameboard {
			cells: [0; SIZE],
			possible_moves: [0; SIZE],
			lines: [[[Line {score,representation}; 4]; SIZE]; SIZE],
            selected_move: None,
            last_move: None,
			black_captures: 0,
			white_captures: 0,
			value: 0,
			result: None,
		}
	}

	pub fn is_finish(&self) -> bool {
		self.result.is_some()
	}
}

impl Gameboard {
	pub fn count_tree(&self, tree_lines: [u32; 4], stone: u8) -> u8 {
		let tree_forms: [u16; 4] = get_tree_forms!(stone);
		tree_lines.iter().fold(0, |nbr_tree, line| {
			if (0..6).any(|range| {
				let line_to_check: u32 = line >> (range * 2);
				tree_forms.contains(&(concat_stones!(line_to_check, 6) as u16))
			}) {
				return nbr_tree + 1;
			}
			nbr_tree
		})
	}

	pub fn count_capture(&mut self, capture_lines: [(u8, (isize, isize)); 8], x: usize, y: usize, stone: u8) -> u8 {
		let capture_form: u8 = get_capture_form!(stone);
		capture_lines.iter().fold(0, |nbr_capture, (line, coef)| {
			if *line == capture_form {
				self.clear_stone((x as isize + 1 * coef.0) as usize, (y as isize + 1 * coef.1) as usize);
				self.clear_stone((x as isize + 2 * coef.0) as usize, (y as isize + 2 * coef.1) as usize);
				return nbr_capture + 1;
			}
			nbr_capture
		})
	}

	pub fn clear_stone(&mut self, x: usize, y: usize) {
		self.cells[x] &= clear_stone!(y);
		self.update_neighbors(x as isize, y as isize);
	}

	pub fn update_neighbors(&mut self, x: isize, y: isize) {
		if get_stone!(self.cells[x as usize], y as usize) == NOPE {
			return;
		}
		self.value -= self.lines[x as usize][y as usize].iter().fold(0, |sum, line| { sum + line.score});
		let x_min = (x - 5).max(0) as usize;
		let x_max = (x + 5).min(SIZE as isize - 1) as usize;
		let y_min = (y - 5).max(0) as usize;
		let y_max = (y + 5).min(SIZE as isize - 1) as usize;
		let diago_up_left = (y as usize - y_min).min(x as usize - x_min);
		let diago_up_right = (y as usize - y_min).min(x_max - x as usize);
		let diago_down_right = (y_max - y as usize).min(x_max - x as usize);
		let diago_down_left = (y_max - y as usize).min(x as usize - x_min);

		let lines: [(u64, (isize, isize)); 4] = update_lines!(self.cells, x as usize, x_min, x_max, y as usize, y_min, y_max, diago_up_left, diago_down_right, diago_down_left, diago_up_right);
		lines.iter().enumerate().for_each(|(i, (line, coef))| {
			let mut tmp_x = x;
			let mut tmp_y = y;
			let mut tmp_line = *line;
			while tmp_line != 0 {
				tmp_x = tmp_x + coef.0;
				tmp_y = tmp_y + coef.1;
				let value = evale_one_line(tmp_line);
				self.lines[tmp_x as usize][tmp_y as usize][i].score = value;
				self.lines[tmp_x as usize][tmp_y as usize][i].representation = 0;//Todo to change
				self.value += value;
				tmp_line = tmp_line >> 2;
			}
		})
	}

	pub fn try_make_move(&mut self, x: isize, y: isize, stone: u8) -> bool {
		let x_min = (x - 5).max(0) as usize;
		let x_max = (x + 5).min(SIZE as isize - 1) as usize;
		let y_min = (y - 5).max(0) as usize;
		let y_max = (y + 5).min(SIZE as isize - 1) as usize;
		let diago_up_left = (y as usize - y_min).min(x as usize - x_min);
		let diago_up_right = (y as usize - y_min).min(x_max - x as usize);
		let diago_down_right = (y_max - y as usize).min(x_max - x as usize);
		let diago_down_left = (y_max - y as usize).min(x as usize - x_min);

		let capture_lines: [(u8, (isize, isize)); 8] = capture_lines!(self.cells, x as usize, x_min, x_max, y as usize, y_min, y_max, diago_up_left, diago_down_right, diago_down_left, diago_up_right);
		let nbr_capture = self.count_capture(capture_lines, x as usize, y as usize, stone);
		if nbr_capture == 0 {
			let tree_lines: [u32; 4] = tree_lines!(self.cells, x as usize, x_min, x_max, y as usize, y_min, y_max, diago_up_left, diago_down_right, diago_down_left, diago_up_right);
			let nbr_tree = self.count_tree(tree_lines, stone);
			return nbr_tree < 2;
		}
		match stone {
			BLACK => self.black_captures += nbr_capture << 1,
			_ => self.white_captures += nbr_capture << 1,
		}
		true
	}

	pub fn make_move(&mut self, x: usize, y: usize, stone: u8) -> bool {
		if !self.is_finish() && get_stone!(self.cells[x], y) == NOPE {
			self.cells[x] |= set_stone!(y, stone);
			if self.try_make_move(x as isize, y as isize, stone) {
				self.update_result(x, y);
				self.update_possible_move(x as isize, y as isize);
				self.last_move = Some((x, y));
				self.selected_move = None;
				return true;
			}
			self.cells[x] &= clear_stone!(y);
        }
        false
    }
	
	pub fn update_possible_move(&mut self, x: isize, y: isize) {
		let min_x = (x - 1).max(0) as usize;
		let min_y = (y - 1).max(0) as usize;
		let max_x = (x + 1).min(SIZE as isize - 1) as usize;
		let max_y = (y + 1).min(SIZE as isize - 1) as usize;

		let x = x as usize;
		let y = y as usize;
		let moves = [(min_x, y), (min_x, min_y), (min_x, max_y), (max_x, y), (max_x, min_y), (max_x, max_y), (x, min_y), (x, max_y)];
		moves
			.iter()
			.for_each(|new_move| {
				if get_stone!(self.cells[new_move.0], new_move.1) == NOPE {
					self.possible_moves[new_move.0 as usize] |= set_move!(new_move.1)
				}
			})
	}
	
	pub fn expand(&self) -> Vec<(usize, usize)> {
		(0..SIZE)
			.flat_map(|y| {
				(0..SIZE)
				.filter(move |&x| self.possible_moves[x] >> y & 0b1 == 1 && get_stone!(self.cells[x], y) == NOPE)
				.map(move |x| (x, y))
			})
		.collect()
	}

	pub fn update_result(&mut self, x: usize, y: usize) {
		if self.black_captures >= 10 {
			self.result = Some(GameResult::BlackWin);
			self.value = -10000000;
		}
		else if self.white_captures >= 10 {
			self.result = Some(GameResult::WhiteWin);
			self.value = 10000000;
		}
		else {
			let x_min = (x as isize - 5).max(0) as usize;
			let x_max = (x + 5).min(SIZE - 1);
			let y_min = (y as isize  - 5).max(0) as usize;
			let y_max = (y + 5).min(SIZE - 1);

			let diago_up_left = (y as usize - y_min).min(x as usize - x_min);
			let diago_up_right = (y as usize - y_min).min(x_max - x as usize);
			let diago_down_right = (y_max - y as usize).min(x_max - x as usize);
			let diago_down_left = (y_max - y as usize).min(x as usize - x_min);
			let lines: [u32; 4] = tree_lines!(self.cells, x as usize, x_min, x_max, y as usize, y_min, y_max, diago_up_left, diago_down_right, diago_down_left, diago_up_right);
			lines.iter().any(|line| {
				(0..8).any(|range| {
					let tmp_line: u16 = concat_stones!((line >> (range * 2)) as u32, 5) as u16;
					return match tmp_line {
						WHITE_5_ALIGN => {
							self.result = Some(GameResult::WhiteWin);
							true
						},
						BLACK_5_ALIGN => {
							self.result = Some(GameResult::BlackWin);
							true
						}
						_ => {
							false
						}
					};
				})
			});
		}
	}
}

impl PartialOrd for Gameboard {
    fn partial_cmp(&self, other: &Gameboard) -> Option<Ordering> {
        other.value.partial_cmp(&self.value)
    }
}

impl PartialEq for Gameboard {
    fn eq(&self, other: &Gameboard) -> bool {
        self.cells == other.cells && self.black_captures == other.black_captures && self.white_captures == other.white_captures
    }
}

impl Hash for Gameboard {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.cells.hash(state);
        self.black_captures.hash(state);
        self.white_captures.hash(state);
    }
}

impl PartialEq for Line {
    fn eq(&self, other: &Line) -> bool {
        self.score == other.score && self.representation == other.representation
    }
}
