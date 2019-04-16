use crate::models::gameboard::*;

const BLACK_WHITE: u16 =			0b00_00_00_01_10_00;
const WHITE_BLACK: u16 =			0b00_00_00_10_01_00;
const EMPTY: u16 =					0b00_00_00_00_00_00;
const ONE_BLACK: u16 =				0b00_00_00_00_01_00;
const ONE_WHITE: u16 =				0b00_00_00_00_10_00;
const FOUR_BLACK: u16 =				0b00_01_01_01_01_00;
const FOUR_WHITE: u16 =				0b00_10_10_10_10_00;

const THREE_BLACK_CLOSE1: u16 =		0b00_00_01_01_01_10;
const THREE_BLACK_CLOSE2: u16 =		0b10_01_00_01_01_00;
const THREE_BLACK_CLOSE3: u16 =		0b10_01_01_01_00_00;

const THREE_WHITE_CLOSE1: u16 =		0b00_00_10_10_10_01;
const THREE_WHITE_CLOSE2: u16 =		0b01_10_00_10_10_00;
const THREE_WHITE_CLOSE3: u16 =		0b01_10_10_10_00_00;

const FOUR_BLACK_CLOSE1: u16 =		0b10_01_00_01_01_01;
const FOUR_BLACK_CLOSE2: u16 =		0b10_01_01_01_00_01;
const FOUR_BLACK_CLOSE3: u16 =		0b10_01_01_01_01_00;
const FOUR_BLACK_CLOSE4: u16 =		0b00_01_01_01_01_10;
const FOUR_BLACK_CLOSE5: u16 =		0b01_00_01_01_01_00;
const FOUR_BLACK_CLOSE6: u16 =		0b01_01_00_01_01_00;
const FOUR_BLACK_CLOSE7: u16 =		0b00_01_01_01_00_01;

const FOUR_WHITE_CLOSE1: u16 =		0b01_10_00_10_10_10;
const FOUR_WHITE_CLOSE2: u16 =		0b01_10_10_10_00_10;
const FOUR_WHITE_CLOSE3: u16 =		0b01_10_10_10_10_00;
const FOUR_WHITE_CLOSE4: u16 =		0b00_10_10_10_10_01;
const FOUR_WHITE_CLOSE5: u16 =		0b10_00_10_10_10_00;
const FOUR_WHITE_CLOSE6: u16 =		0b10_10_00_10_10_00;
const FOUR_WHITE_CLOSE7: u16 =		0b00_10_10_10_00_10;
// const FOUR_WHITE_CLOSE8: u16 =		0b10_10_10_00_10_00;

const TWO_BLACK_OPEN: u16 =			0b00_00_00_01_01_00;
const TWO_WHITE_OPEN: u16 =			0b00_00_00_10_10_00;

const TWO_BLACK_OPEN_HOLE: u16 =	0b00_00_01_00_01_00;
const TWO_WHITE_OPEN_HOLE: u16 =	0b00_00_10_00_10_00;

const THREE_BLACK_OPEN: u16 =		0b00_00_01_01_01_00;
const THREE_WHITE_OPEN: u16 =		0b00_00_10_10_10_00;

const THREE_BLACK_OPEN_HOLE1: u16 =	0b00_01_00_01_01_00;
const THREE_BLACK_OPEN_HOLE2: u16 =	0b00_01_01_00_01_00;

const THREE_WHITE_OPEN_HOLE1: u16 =	0b00_10_00_10_10_00;
const THREE_WHITE_OPEN_HOLE2: u16 =	0b00_10_10_00_10_00;

pub const BLACK_5_ALIGN: u16 =		0b00_01_01_01_01_01;
pub const WHITE_5_ALIGN: u16 =		0b00_10_10_10_10_10;

pub fn evale_one_line(line: u16, stone: u8, black_captures: u8, white_captures: u8) -> i32 {
	let score = match (line & 0b11_11_11_11_11_11) {
		EMPTY | ONE_BLACK | ONE_WHITE | BLACK_WHITE | WHITE_BLACK => {
			0
		},
		align5_white if (align5_white & 0b11_11_11_11_11 == WHITE_5_ALIGN) => {
			10000000
		},
		align5_black if (align5_black & 0b11_11_11_11_11 == BLACK_5_ALIGN) => {
			-10000000
		},
		FOUR_WHITE => {
			100000
		},
		FOUR_BLACK => {
			-100000
		},
		THREE_WHITE_CLOSE1 |
		THREE_WHITE_CLOSE2 |
		THREE_WHITE_CLOSE3 => {
				100
		},
		THREE_BLACK_CLOSE1 |
		THREE_BLACK_CLOSE2 |
		THREE_BLACK_CLOSE3 => {
			-100
		},
		FOUR_WHITE_CLOSE1 |
		FOUR_WHITE_CLOSE2 |
		FOUR_WHITE_CLOSE3 |
		FOUR_WHITE_CLOSE4 |
		FOUR_WHITE_CLOSE5 |
		FOUR_WHITE_CLOSE6 |
		FOUR_WHITE_CLOSE7 => {
			10000
		},
		FOUR_BLACK_CLOSE1 |
		FOUR_BLACK_CLOSE2 |
		FOUR_BLACK_CLOSE3 |
		FOUR_BLACK_CLOSE4 |
		FOUR_BLACK_CLOSE5 |
		FOUR_BLACK_CLOSE6 |
		FOUR_BLACK_CLOSE7
			=> {
			-10000
		},
		align2white_open if align2white_open & 0b00_11_11_11_11_11 == TWO_WHITE_OPEN => {
			100
		},
		align2black_open if align2black_open & 0b00_11_11_11_11_11 == TWO_BLACK_OPEN => {
			-100
		},
		align2white_hole if align2white_hole & 0b00_11_11_11_11_11 == TWO_WHITE_OPEN_HOLE => {
			10
		},
		align2black_hole if align2black_hole & 0b00_11_11_11_11_11 == TWO_BLACK_OPEN_HOLE => {
			-10
		},
		align3white if (align3white & 0b00_11_11_11_11_11) == THREE_WHITE_OPEN => {
				10000
		}
		align3black if (align3black & 0b00_11_11_11_11_11) == THREE_BLACK_OPEN => {
			-10000
		},
		align3white if (align3white & 0b00_11_11_11_11_11) == THREE_WHITE_OPEN_HOLE1
					|| (align3white & 0b00_11_11_11_11_11) == THREE_WHITE_OPEN_HOLE2 => {
				1000
		}
		align3black if (align3black & 0b00_11_11_11_11_11) == THREE_BLACK_OPEN_HOLE1
					|| (align3black & 0b00_11_11_11_11_11) == THREE_BLACK_OPEN_HOLE2 => {
			-1000
		},
		white_capturable if (white_capturable & 0b00_00_00_11_11_11) == 0b00_00_00_10_10_01
						|| (white_capturable & 0b00_00_11_11_11_11) == 0b00_00_01_10_10_00 => {
			-(10_i32.pow((black_captures as u32 / 2) + 1))
		}
		black_capturable if (black_capturable & 0b00_00_00_11_11_11) == 0b00_00_00_01_01_10
						|| (black_capturable & 0b00_00_11_11_11_11) == 0b00_00_10_01_01_00 => {
			(10_i32.pow((white_captures as u32 / 2) + 1))
		}
		_ => 0,
	};
	if (score < 0 && stone == WHITE) || (score > 0 && stone == BLACK) {
		score * 10
	}
	else {
		score
	}
}








