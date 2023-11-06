fn get_priority(letter: u8) -> u8 {
    match (letter as u8) < ('a' as u8) {
        true => letter as u8 - 'A' as u8 + 26,
        false => letter as u8 - 'a' as u8,
    }
}

fn priority_from_mask(mut mask: u64) -> u64 {
    let mut p = 1;
    while mask & 1 == 0 {
        mask >>= 1;
        p += 1;
    }
    return p;
}

pub(crate) fn part1(buffer: &[u8]) -> String {
    let mut offset = 0;

    let mut priorities_sum = 0;

    while offset < buffer.len() {
        let start_offset = offset;
        while buffer[offset] != '\n' as u8 { offset += 1 }
        let half_offset = (start_offset + offset) / 2;

        let mut left = 0 as u64;
        for c in start_offset..half_offset {
            left |= 1 << get_priority(buffer[c]);
        }

        let mut right = 0 as u64;
        for c in half_offset..offset {
            right |= 1 << get_priority(buffer[c]);
        }

        let shared = left & right;
        if shared > 0 {
            priorities_sum += priority_from_mask(shared);
        }

        offset += 1;
    }

    return priorities_sum.to_string();
}

pub(crate) fn part2(buffer: &[u8]) -> String {
    let mut offset = 0;

    let mut priorities_sum = 0;

    while offset < buffer.len() {
        let mut shared = u64::MAX;
        for _ in 0..3 {
            let mut current = 0;
            while buffer[offset] != '\n' as u8 {
                current |= 1 << get_priority(buffer[offset]);
                offset += 1
            }
            shared &= current;
            offset += 1;
        }

        priorities_sum += priority_from_mask(shared);
    }

    return priorities_sum.to_string();
}