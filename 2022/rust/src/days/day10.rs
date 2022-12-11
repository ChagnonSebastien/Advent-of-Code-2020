use crate::parser::read_signed_int;

fn cpu_tick(buffer: &[u8], offset: &mut usize) -> isize {
    match buffer[*offset] as char {
        'a' | 'n' => {
            *offset += 5;
            0
        }
        _ => {
            let amount = read_signed_int(buffer, offset).unwrap();
            *offset += 1;
            amount
        }
    }
}

pub(crate) fn part1(buffer: &[u8]) -> String {
    let mut offset = 0;

    let mut x: isize = 1;
    let mut tick: isize = 1;

    let mut signal_sum = 0;

    while tick < 220 {
        x += cpu_tick(buffer, &mut offset);
        tick += 1;
        if (tick + 20) % 40 == 0 {
            signal_sum += tick * x;
        }
    }

    return signal_sum.to_string()
}

pub(crate) fn part2(buffer: &[u8]) -> String {
    let mut offset = 0;

    let mut x: isize = 1;
    let mut tick: isize = 0;

    let mut display = String::with_capacity((40 + 1) * 6);

    while tick < 240 {
        if tick % 40 == 0 {
            display.push('\n');
        }

        let pixel = match x.abs_diff(tick % 40) <= 1 {
            true => '#',
            false => ' ',
        };
        display.push(pixel);

        x += cpu_tick(buffer, &mut offset);
        tick += 1;
    }

    return display;
}