use crate::parser::read_unsigned_int;

fn parse_assignement(buffer: &[u8], offset: &mut usize) -> (usize, usize, usize, usize) {
    let start_a = read_unsigned_int(buffer, offset).unwrap();
    *offset += 1;
    let end_a = read_unsigned_int(buffer, offset).unwrap();
    *offset += 1;
    let start_b = read_unsigned_int(buffer, offset).unwrap();
    *offset += 1;
    let end_b = read_unsigned_int(buffer, offset).unwrap();
    *offset += 1;
    return (start_a, end_a, start_b, end_b);
}

pub(crate) fn part1(input: &String) -> String {
    let buffer = input.as_bytes();
    let mut offset = 0;

    let mut contained = 0;
    while offset < buffer.len() {
        let (start_a, end_a, start_b, end_b) = parse_assignement(buffer, &mut offset);
        if start_b >= start_a && end_b <= end_a || start_a >= start_b && end_a <= end_b {
            contained += 1;
        }
    }

    return contained.to_string()
}

pub(crate) fn part2(input: &String) -> String {
    let buffer = input.as_bytes();
    let mut offset = 0;

    let mut overlaping = 0;
    while offset < buffer.len() {
        let (start_a, end_a, start_b, end_b) = parse_assignement(buffer, &mut offset);
        if !(end_a < start_b || end_b < start_a) {
            overlaping += 1;
        }
    }

    return overlaping.to_string()
}