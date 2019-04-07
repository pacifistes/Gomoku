use crate::models::game::GameResult;
use crate::eval::evale_one_line;
use std::hash::{Hash, Hasher};
use std::cmp::Ordering;

/// Size of game board.
pub const SIZE: usize = 19;

pub const NOPE: u8 = 0b00;
pub const BLACK: u8 = 0b01;
pub const WHITE: u8 = 0b10;

pub const NEIGHBORS_DIRECTIONS: [(isize, isize); 4] = [(0, -1), (-1, 0), (-1, -1), (-1, 1)];
pub const MY_DIRECTIONS: [(isize, isize); 4] = [(0, 1), (1, 0), (1, 1), (1, -1)];

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
	pub score: i32,
	pub representation: u16,
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
	pub upperbound: isize,
	pub lowerbound: isize,
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
			upperbound: std::i64::MAX as isize,
			lowerbound: (std::i64::MIN + 1) as isize,
			value: 0,
			result: None,
		}
	}

	pub fn is_finish(&self) -> bool {
		self.result.is_some()
	}
}

impl Gameboard {
	pub fn count_tree(&self, x: isize, y: isize, stone: u8) -> u8 {
		let tree_forms: [u16; 4] = get_tree_forms!(stone);
		let mut nbr_tree = 0;
		let directions: [(isize, isize); 4] = NEIGHBORS_DIRECTIONS;
		directions.iter().enumerate().for_each(|(i, dir)| {
			if (nbr_tree >= 2) {
				return;
			}
			let mut j = 1;
			while j < 5 {
				let tmp_x = x + dir.0 * j;
				let tmp_y = y + dir.1 * j;
				if (tmp_x < 0 || tmp_y < 0 || tmp_y >= SIZE as isize) {
					break;
				}
				if tree_forms.contains(&(self.lines[tmp_x as usize][tmp_y as usize][i].representation)) {
					nbr_tree += 1;
					break;
				}
				j += 1;
			}
		});
		nbr_tree
	}

	pub fn count_capture(&mut self, x: usize, y: usize, stone: u8) -> u8 {
		let capture_form: u8 = get_capture_form!(stone);
		let mut nbr_capture = 0;

		let directions: [(isize, isize); 4] = MY_DIRECTIONS;
		directions.iter().enumerate().for_each(|(i, dir)| {
			if self.lines[x as usize][y as usize][i].representation as u8 == capture_form {
				self.clear_stone((x as isize + 1 * dir.0) as usize, (y as isize + 1 * dir.1) as usize);
				self.clear_stone((x as isize + 2 * dir.0) as usize, (y as isize + 2 * dir.1) as usize);
				nbr_capture += 1;
			}
		});

		let directions: [(isize, isize); 4] = NEIGHBORS_DIRECTIONS;
		directions.iter().enumerate().for_each(|(i, dir)| {
			let tmp_x: isize = x as isize + dir.0 * 3;
			let tmp_y: isize = y as isize + dir.1 * 3;
			if (tmp_x < 0 || tmp_y < 0 || tmp_y >= SIZE as isize) {
				return;
			}
			if self.lines[tmp_x as usize][tmp_y as usize][i].representation as u8 == capture_form {
				self.clear_stone((x as isize + 1 * dir.0) as usize, (y as isize + 1 * dir.1) as usize);
				self.clear_stone((x as isize + 2 * dir.0) as usize, (y as isize + 2 * dir.1) as usize);
				nbr_capture += 1;
			}
		});
		nbr_capture
	}
}

impl Gameboard {
	pub fn try_make_move(&mut self, x: isize, y: isize, stone: u8) -> bool {
		let nbr_capture = self.count_capture(x as usize, y as usize, stone);
		if nbr_capture == 0 {
			let nbr_tree = self.count_tree(x, y, stone);
			return nbr_tree < 2;
		}
		// self.value -= (self.white_captures as isize * self.white_captures as isize * 100) - (self.black_captures as isize * self.black_captures as isize * 100);
		match stone {
			BLACK => self.black_captures += nbr_capture * 2,
			_ => self.white_captures += nbr_capture * 2,
		}
		// self.value += (self.white_captures as isize * self.white_captures as isize * 100) - (self.black_captures as isize * self.black_captures as isize * 100);
		true
	}

	pub fn make_move(&mut self, x: usize, y: usize, stone: u8) -> bool {
		if !self.is_finish() && get_stone!(self.cells[x], y) == NOPE {
			self.cells[x] |= set_stone!(y, stone);
			self.update_neighbors(x as isize, y as isize, stone);
			if self.try_make_move(x as isize, y as isize, stone) {
				self.update_result(x as isize, y as isize);
				self.update_possible_move(x as isize, y as isize);
				self.last_move = Some((x, y));
				self.selected_move = None;
				return true;
			}
			self.clear_stone(x, y);
			// self.cells[x] &= clear_stone!(y);
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
}

impl Gameboard {
	pub fn expand(&self) -> Vec<(usize, usize)> {
		(0..SIZE)
			.flat_map(|y| {
				(0..SIZE)
				.filter(move |&x| self.possible_moves[x] >> y & 0b1 == 1 && get_stone!(self.cells[x], y) == NOPE)
				.map(move |x| (x, y))
			})
		.collect()
	}

	pub fn clear_stone(&mut self, x: usize, y: usize) {
		self.cells[x] &= clear_stone!(y);
		self.update_neighbors(x as isize, y as isize, NOPE);
	}

	pub fn update_neighbors(&mut self, x: isize, y: isize, stone: u8) {
		self.value -= self.lines[x as usize][y as usize].iter().fold(0, |sum, line| { sum + line.score}) as isize;
		(0..4).for_each(|i| {
			self.lines[x as usize][y as usize][i].representation =
			(self.lines[x as usize][y as usize][i].representation & !(0b11)) | stone as u16;

			let value = evale_one_line(self.lines[x as usize][y as usize][i].representation, stone, self.black_captures, self.white_captures);
			self.lines[x as usize][y as usize][i].score = value;
			self.value += value as isize;
		});

		let directions: [(isize, isize); 4] = NEIGHBORS_DIRECTIONS;
		directions.iter().enumerate().for_each(|(i, dir)| {
			let mut j = 1;
			while j < 6 {
				let tmp_x = x + dir.0 * j;
				let tmp_y = y + dir.1 * j;
				if (tmp_x < 0 || tmp_y < 0 || tmp_y >= SIZE as isize) {
					break;
				}
				self.lines[tmp_x as usize][tmp_y as usize][i].representation =
				(self.lines[tmp_x as usize][tmp_y as usize][i].representation & !(0b11 << (j * 2))) | ((stone as u16) << (j * 2));
				self.value -= self.lines[tmp_x as usize][tmp_y as usize][i].score as isize;
				let value = evale_one_line(self.lines[tmp_x as usize][tmp_y as usize][i].representation, stone, self.black_captures, self.white_captures);
				self.lines[tmp_x as usize][tmp_y as usize][i].score = value;
				self.value += value as isize;
				j += 1;
			}
		})
	}

	pub fn update_result(&mut self, x: isize, y: isize) {
		if self.black_captures >= 10 {
			self.result = Some(GameResult::BlackWin);
			self.value = -10000000;
		}
		else if self.white_captures >= 10 {
			self.result = Some(GameResult::WhiteWin);
			self.value = 10000000;
		}
		else {
			let directions: [(isize, isize); 4] = NEIGHBORS_DIRECTIONS;
			directions.iter().enumerate().any(|(i, dir)| {
				let mut j = 0;
				while j < 5 {
					let tmp_x = x + dir.0 * j;
					let tmp_y = y + dir.1 * j;
					if (tmp_x < 0 || tmp_y < 0 || tmp_y >= SIZE as isize) {
						break;
					}
					match (self.lines[tmp_x as usize][tmp_y as usize][i].representation & 0b11_11_11_11_11) {
						WHITE_5_ALIGN => {
							self.result = Some(GameResult::WhiteWin);
							return true;
							// check_winning!(self, x, y, GameResult::WhiteWin, stone)
						},
						BLACK_5_ALIGN => {
							self.result = Some(GameResult::BlackWin);
							return true;
							// check_winning!(self, x, y, GameResult::BlackWin, stone)
						},
						_ => (),
					}
					j += 1;
				}
				false
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
        // self.black_captures.hash(state);
        // self.white_captures.hash(state);
    }
}

impl PartialEq for Line {
    fn eq(&self, other: &Line) -> bool {
        self.score == other.score && self.representation == other.representation
    }
}
