use crate::models::gameboard::*;

	pub fn evale_one_line(line: u64) -> isize {
		match (line & 0b11_11_11_11_11_11) as u16 {
			// _xxxxx
			align5_white if (align5_white & 0b11_11_11_11_11 == 0b10_10_10_10_10) => {
				// println!(": [02]RETURN 10000000 align5 white");
				10000000
			},
			// _ooooo
			align5_black if (align5_black & 0b11_11_11_11_11 == 0b01_01_01_01_01) => {
				// dbg_line((line & 0b1111_1111_1111) as u16);
				// println!(": [03]RETURN - 10000000 align5 black");
				-10000000
			},
			// .oooo.
			0b00_01_01_01_01_00 => {
				// dbg_line((line & 0b1111_1111_1111) as u16);
				// println!(": [04]value -= 100000 align4 open black");
				-100000
			},
			// .xxxx.
			0b00_10_10_10_10_00 => {
				// dbg_line((line & 0b1111_1111_1111) as u16);
				// println!(": [05]value += 100000 align4 open white");
				100000
			},
			// ..ooox
			// .o.oox
			// xo.oo.
			// xooo..
			0b00_00_01_01_01_10 | 
			0b00_01_00_01_01_10 | 
			0b10_01_00_01_01_00 | 
			0b10_01_01_01_00_00 => {
				// dbg_line((line & 0b1111_1111_1111) as u16);
				// println!(": [06]value -= 100 align3 close black");
				-100
			},
			// ..xxxo
			// .x.xxo
			// ox.xx.
			// oxxx..
			0b00_00_10_10_10_01 |
			0b00_10_00_10_10_01 |
			0b01_10_00_10_10_00 |
			0b01_10_10_10_00_00  => {
				// dbg_line((line & 0b1111_1111_1111) as u16);
				// println!(": [07]Value += 100 align3 close white");
				100
			},
			//xo.ooo
			//xoo.oo
			//xooo.o
			//xoooo.
			//.oooox
			//o.ooo.
			//oo.oo.
			//ooo.o.
			// .ooo.o

			0b10_01_00_01_01_01 |
			0b10_01_01_00_01_01 |
			0b10_01_01_01_00_01 |
			0b10_01_01_01_01_00 |
			0b00_01_01_01_01_10 |
			0b01_00_01_01_01_00 |
			0b01_01_00_01_01_00 |
			0b01_01_01_00_01_00 | 
			0b00_01_01_01_00_01 => {
			// dbg_line((line & 0b1111_1111_1111) as u16);
			// println!(": [08]value -= 10000 align4 close black");
				-10000
			},
			//ox.xxx
			//oxx.xx
			//oxxx.x
			//oxxxx.
			//x.xxx.
			//xx.xx.
			//xxx.x.

			// .xxx.x
			0b01_10_00_10_10_10 |
			0b01_10_10_00_10_10 |
			0b01_10_10_10_00_10 |
			0b01_10_10_10_10_00 |
			0b00_10_10_10_10_01 |

			0b10_00_10_10_10_00 |
			0b10_10_00_10_10_00 |
			0b10_10_10_00_10_00 |
			0b00_10_10_10_00_10 => {
			// dbg_line((line & 0b1111_1111_1111) as u16);
			// println!(": [09]value += 10000 align4 close white");
				10000
			},
			//_..oo.
			align2black_open if align2black_open & 0b00_11_11_11_11_11 == 0b00_00_00_01_01_00 => {
				// dbg_line((line & 0b1111_1111_1111) as u16);
				// println!(": [10]value -= 100 align2 open black");
				-100
			},
			//_..xx.
			align2white_open if align2white_open & 0b00_11_11_11_11_11 == 0b00_00_00_10_10_00 => {
				// dbg_line((line & 0b1111_1111_1111) as u16);
				// println!(": [11]Value += 100 align2 open white");
				100
			},
			// //...xxo
			// //oxx...
			// align2black_close if align2black_close & 0b00_11_11_11_11_11 == 0b00_00_00_01_01_10
			// 			|| align2black_close & 0b11_11_11_11_11_11 == 0b10_01_01_00_00_00 => {
			// 	// dbg_line((line & 0b1111_1111_1111) as u16);
					// // println!(": [12]value -= 100 align2 close black");
			// 	value -= 50;
			// 		j = 8;
			// },
			// //...oox
			// //xoo...
			// align2white_close if align2white_close & 0b00_11_11_11_11_11 == 0b00_00_00_10_10_01
			// 			|| align2white_close & 0b11_11_11_11_11_11 == 0b01_10_10_00_00_00 => {
			// 	// dbg_line((line & 0b1111_1111_1111) as u16);
				// // println!(": [13]value += 10 align2 close white");
			// 	value += 50;
			// 		j = 8;
			// },
			//_.o.o.
			align2black_hole if align2black_hole & 0b00_11_11_11_11_11 == 0b00_00_01_00_01_00 => {
				// println!(": [14]value -= 10 align2 hole black");
				-10
			},
			//_.x.x.
			align2white_hole if align2white_hole & 0b00_11_11_11_11_11 == 0b00_00_10_00_10_00 => {
				// dbg_line((line & 0b1111_1111_1111) as u16);
				// println!(": [15]Value += 10 align2 hole white");
				10
			},
			// _.ooo.
			align3black if (align3black & 0b00_11_11_11_11_11) == 0b00_00_01_01_01_00 => {

				// dbg_line((line & 0b1111_1111_1111) as u16);
				// println!(": [16]value -= 10000 align3 open black");
				-10000
			},
			// _oo.o.
			// _o.oo.
			align3black if (align3black & 0b00_11_11_11_11_11) == 0b00_01_01_00_01_00
						|| (align3black & 0b00_11_11_11_11_11) == 0b00_01_00_01_01_00 => {
				// println!(": [17]value -= 10000 align3 open black");
				-10000
			},
			// _.xxx.
			align3white if (align3white & 0b00_11_11_11_11_11) == 0b00_00_10_10_10_00 => {
				// println!(": [18]Value += 1000 align3 open white");
				1000
			}
			// _xx.x.
			// _x.xx.
			align3white if (align3white & 0b00_11_11_11_11_11) == 0b00_10_10_00_10_00
						|| (align3white & 0b00_11_11_11_11_11) == 0b00_10_00_10_10_00 => {
				// dbg_line((line & 0b1111_1111_1111) as u16);
				// println!(": [19]Value += 1000 align3 open white");
				1000
			}
			_ => 0,
		}
	}
// }

