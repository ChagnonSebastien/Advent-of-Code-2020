const AMOUNT_BITS: usize = 12;

fn parse_binary_number(buffer: &[u8], cursor: &mut usize) -> u16 {
    let mut value = 0 as u16;
    for _ in 0..AMOUNT_BITS {
        value <<= 1;
        value |= (buffer[*cursor] - '0' as u8) as u16;
        *cursor += 1;
    }
    return value;
}

pub(crate) fn part1(buffer: &[u8]) -> String {
    let mut cursor = 0;
    while cursor < buffer.len() {
        let i = parse_binary_number(buffer, &mut cursor);
        cursor += 1;
    }

    return "".to_string()
}

pub(crate) fn part2(buffer: &[u8]) -> String {
    let mut cursor = 0;



    return "".to_string()
}