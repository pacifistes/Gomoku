use crate::models::gameboard::*;

pub fn evale_one_line(line: u16, stone: u8, black_captures: u8, white_captures: u8) -> i32 {
	let score = match (line & 0b11_11_11_11_11_11) {
		0b00_00_00_00_00_00 |
		// 0b00_00_00_00_00_01 |
		// 0b00_00_00_00_00_10 |
		0b00_00_00_00_01_00 |
		0b00_00_00_00_10_00 |
		// 0b00_00_00_01_00_00 |
		// 0b00_00_00_10_00_00 |
		// 0b00_00_01_00_00_00 |
		// 0b00_00_10_00_00_00 |
		// 0b00_01_00_00_00_00 |
		// 0b00_10_00_00_00_00 |
		// 0b01_00_00_00_00_00 |
		// 0b10_00_00_00_00_00 |
		0b00_00_00_01_10_00 |
		0b00_00_00_10_01_00 => {
			0
		}
		align5_white if (align5_white & 0b11_11_11_11_11 == 0b10_10_10_10_10) => {
			10000000
		},
		align5_black if (align5_black & 0b11_11_11_11_11 == 0b01_01_01_01_01) => {
			-10000000
		},
		align4_white_open if align4_white_open == 0b00_10_10_10_10_00 => {
			100000
		},
		align4_black_open if align4_black_open == 0b00_01_01_01_01_00 => {
			-100000
		},
		// 0b00_10_10_10_00_00 |
		// 0b00_10_10_00_10_00 |
		// 0b00_10_00_10_10_00 |
		// 0b00_00_10_10_10_00 => {
		// 	20000
			
		// },
		// 0b00_01_01_01_00_00 |
		// 0b00_01_01_00_01_00 |
		// 0b00_01_00_01_01_00 |
		// 0b00_00_01_01_01_00  => {
		// 	-20000
		// },
		align3white if (align3white & 0b00_11_11_11_11_11) == 0b00_00_10_10_10_00
					/*|| (align3white & 0b00_11_11_11_11_11) == 0b00_10_10_10_00_00*/ => {
			10000
		},
		align3black if (align3black & 0b00_11_11_11_11_11) == 0b00_00_01_01_01_00
					/*|| (align3black & 0b00_11_11_11_11_11) == 0b00_01_01_01_00_00*/ => {
			-10000
		},
		0b10_01_00_01_01_01 |
		0b10_01_01_01_00_01 |
		0b10_01_01_01_01_00 |
		0b00_01_01_01_01_10 |
		0b01_00_01_01_01_00 |
		0b01_01_00_01_01_00 |
		0b01_01_01_00_01_00 | 
		0b00_01_01_01_00_01 //|
		// 0b10_01_01_00_01_01 | Capturable
		 => {
			-10000
		},
		0b01_10_00_10_10_10 |
		// 0b01_10_10_00_10_10 | Capturable
		0b01_10_10_10_00_10 |
		0b01_10_10_10_10_00 |
		0b00_10_10_10_10_01 |
		0b10_00_10_10_10_00 |
		0b10_10_00_10_10_00 |
		0b10_10_10_00_10_00 |
		0b00_10_10_10_00_10 => {
			10000
		},
		0b00_00_01_01_01_10 | 
		// 0b00_01_00_01_01_10 | Capturable 
		0b10_01_00_01_01_00 | 
		0b10_01_01_01_00_00 => {
			-100
		},
		0b00_00_10_10_10_01 |
		// 0b00_10_00_10_10_01 | Capturable
		0b01_10_00_10_10_00 |
		0b01_10_10_10_00_00  => {
			100
		},
		white_capturable if (white_capturable & 0b00_00_00_11_11_11) == 0b00_00_00_10_10_01
						|| (white_capturable & 0b00_00_11_11_11_11) == 0b00_00_01_10_10_00 => {
			-(10_i32.pow((black_captures as u32 / 2) + 1))
		}
		black_capturable if (black_capturable & 0b00_00_00_11_11_11) == 0b00_00_00_01_01_10
						|| (black_capturable & 0b00_00_11_11_11_11) == 0b00_00_10_01_01_00 => {
			(10_i32.pow((white_captures as u32 / 2) + 1))
		}
		align3black if (align3black & 0b00_11_11_11_11_11) == 0b00_01_01_00_01_00
					|| (align3black & 0b00_11_11_11_11_11) == 0b00_01_00_01_01_00 => {
			-1000
		},
		align3white if (align3white & 0b00_11_11_11_11_11) == 0b00_10_10_00_10_00
					|| (align3white & 0b00_11_11_11_11_11) == 0b00_10_00_10_10_00 => {
			1000
		}
		//_..oo.
		align2black_open if align2black_open & 0b00_11_11_11_11_11 == 0b00_00_00_01_01_00 => {
			-100
		},
		align2white_open if align2white_open & 0b00_11_11_11_11_11 == 0b00_00_00_10_10_00 => {
			100
		},
		align2black_hole if align2black_hole & 0b00_11_11_11_11_11 == 0b00_00_01_00_01_00 => {
			-10
		},
		align2white_hole if align2white_hole & 0b00_11_11_11_11_11 == 0b00_00_10_00_10_00 => {
			10
		},
		_ => 0,
	};
	if (score < 0 && stone == WHITE) || (score > 0 && stone == BLACK) {
		score * 10
	}
	else {
		score
	}
}

// pub fn eval(state: &Gameboard) -> isize {
// 	match true {
// 		black_win if state.black_captures >= 10 => -10000000,
// 		white_win if state.black_captures >= 10 => 10000000,
// 		_ => 1000000,
// 	}
// }








