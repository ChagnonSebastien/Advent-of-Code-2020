use crate::array_utils::sum_n;
use crate::parser::read_unsigned_int;

pub(crate) fn part1(buffer: &[u8]) -> String {
    let mut cursor = 0;
    let mut previous_depth = read_unsigned_int(buffer, &mut cursor);
    cursor += 1;

    let mut increasing_times = 0;
    while cursor < buffer.len() {
        let depth = read_unsigned_int(buffer, &mut cursor);
        cursor += 1;
        if depth > previous_depth {
            increasing_times += 1;
        }
        previous_depth = depth;
    }
    return increasing_times.to_string()
}

pub(crate) fn part2(buffer: &[u8]) -> String {
    let mut cursor = 0;
    let mut previous_depths = [0; 3];
    previous_depths[0] = read_unsigned_int(buffer, &mut cursor);
    cursor += 1;
    previous_depths[1] = read_unsigned_int(buffer, &mut cursor);
    cursor += 1;
    previous_depths[2] = read_unsigned_int(buffer, &mut cursor);
    cursor += 1;
    let mut previous_depth = sum_n(&previous_depths, 3);
    let mut items = 0;

    let mut increasing_times = 0;
    while cursor < buffer.len() {
        let mut depth = previous_depth;
        depth -= previous_depths[items];
        previous_depths[items] = read_unsigned_int(buffer, &mut cursor);
        depth += previous_depths[items];
        cursor += 1;
        if depth > previous_depth {
            increasing_times += 1;
        }
        previous_depth = depth;
        items = (items + 1) % 3;
    }
    return increasing_times.to_string()
}