fn score(i_play: u8, outcome: u8) -> u8 {
    return i_play + 1 + outcome * 3;
}

fn part_1_score(they_play: u8, i_play: u8) -> u8 {
    let outcome = (i_play + 4 - they_play) % 3;
    return score(i_play, outcome);
}


pub(crate) fn part1(input: &String) -> String {
    let buffer = input.as_bytes();
    let mut offset = 0;

    let mut total_score = 0;

    while offset < buffer.len() {
        total_score += part_1_score(
            buffer[offset] - 'A' as u8,
            buffer[offset + 2] - 'X' as u8
        ) as usize;
        offset += 4;
    }

    return total_score.to_string();
}


fn part_2_score(they_play: u8, outcome: u8) -> u8 {
    let i_play = (they_play + 2 + outcome) % 3;
    return score(i_play, outcome);
}

pub(crate) fn part2(input: &String) -> String {
    let buffer = input.as_bytes();
    let mut offset = 0;

    let mut total_score = 0;

    while offset < buffer.len() {
        total_score += part_2_score(
            buffer[offset] - 'A' as u8,
            buffer[offset + 2] - 'X' as u8
        ) as usize;
        offset += 4;
    }

    return total_score.to_string();
}