use std::mem::MaybeUninit;
use std::ptr::write;
use crate::parser::{move_to_next_line, read_unsigned_int};

const BOARD_SIDE: usize = 5;
const BOARD_AREA: usize = BOARD_SIDE * BOARD_SIDE;
const AMOUNT_BOARDS: usize = 100;
const AMOUNT_NUMBERS: usize = 100;

const WIN_FILTERS: [u32; 10] = [
    0b10000_10000_10000_10000_10000,
    0b01000_01000_01000_01000_01000,
    0b00100_00100_00100_00100_00100,
    0b00010_00010_00010_00010_00010,
    0b00001_00001_00001_00001_00001,
    0b11111_00000_00000_00000_00000,
    0b00000_11111_00000_00000_00000,
    0b00000_00000_11111_00000_00000,
    0b00000_00000_00000_11111_00000,
    0b00000_00000_00000_00000_11111,
];

pub(crate) fn part1(buffer: &[u8]) -> String {
    unsafe {
        let mut offset = 0;
        #[allow(invalid_value)]
        let mut pulled_numbers: [u8; AMOUNT_NUMBERS] = MaybeUninit::uninit().assume_init();
        for number in &mut pulled_numbers {
            write(number, read_unsigned_int(buffer, &mut offset) as u8);
            offset += 1;
        }

        offset += 1;
        let first_board_offset = offset;


        return "".to_string()
    }
}

pub(crate) fn part2(buffer: &[u8]) -> String {
    let mut offset = 0;
    return "".to_string()
}