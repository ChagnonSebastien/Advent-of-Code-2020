fn index(buffer: &[u8], trigger_span: usize) -> usize {
    let mut offset = 0;

    let mut stop_in = trigger_span;
    let mut moving_past_offset;
    while stop_in > 0 {


        let (diff, lowest_past_offset) = match offset > trigger_span {
            true => (0, offset - trigger_span),
            false => (trigger_span - offset, 0),
        };
        offset += 1;
        moving_past_offset = offset;
        while moving_past_offset > lowest_past_offset {
            moving_past_offset -= 1;

            if buffer[moving_past_offset] == buffer[offset] {
                break
            }
        }

        let potential_stop_in = moving_past_offset - lowest_past_offset + diff;
        if potential_stop_in > stop_in {
            stop_in = potential_stop_in - 1;
        } else {
            stop_in -= 1;
        }
    }

    return offset + 1;
}

pub(crate) fn part1(input: &String) -> String {
    return index(input.as_bytes(), 4).to_string();
}

pub(crate) fn part2(input: &String) -> String {
    return index(input.as_bytes(), 14).to_string();
}
