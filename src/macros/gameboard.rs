macro_rules! get_stone {
	($line: expr, $y: expr) => {
		($line >> ($y * 2) & 0b11) as u8
	};
}

macro_rules! clear_stone {
	($y: expr) => {
		 !(0b11 << ($y * 2) as u64)
	};
}

macro_rules! set_stone {
	($y: expr, $stone: expr) => {
		($stone as u64) << ($y * 2)
	};
}

macro_rules! set_move {
	($y: expr) => {
		0b1 << $y
	};
}

macro_rules! opposite_stone {
	($stone: expr) => {
		!$stone & 0b11
	};
}

macro_rules! get_tree_forms {
	($stone: expr) => {
		match $stone {
			WHITE => WHITE_TREES,
			_ => BLACK_TREES,
		}
	}
}

macro_rules! get_capture_form {
	($stone: expr) => {
		match $stone {
			WHITE => WHITE_CAPTURE,
			_ => BLACK_CAPTURE,
		}
	}
}

macro_rules! concat_stones {
	($line: expr, $nbr_stone: expr) => {
		($line & ((1 << $nbr_stone * 2) - 1))
	}
}

macro_rules! printboard {
	($cells: expr) => {
		print!("BOARD:\n   ");
		for x in 0..SIZE { print!("{0: <2} ", x) };
		println!();

		for y in 0..SIZE {
			print!("{0: <2} ", y);
			for x in 0..SIZE {
				match get_stone!($cells[x], y) {
					WHITE => print!("W  "),
					BLACK => print!("B  "),
					_ => print!(".  ")
				}
			}
			println!();
		}
	};
}
