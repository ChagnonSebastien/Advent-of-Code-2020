use crate::parser::read_unsigned_int;

fn read_elf_backpack_calories(buffer: &[u8], offset: &mut usize) -> Result<usize, &'static str> {
    let mut calories = 0;

    loop {
        calories = match read_unsigned_int(buffer, offset) {
            Ok(new_amount) => calories + new_amount,
            Err(e) => return Err(e)
        };

        *offset += 1;
        if *offset >= buffer.len() || buffer[*offset] == '\n' as u8 {
            *offset += 1;
            return Ok(calories);
        }
    }
}

pub(crate) fn part1(input: &String) -> String {
    let buffer = input.as_bytes();
    let mut offset = 0;

    let mut max_calories = 0;
    while offset < input.len() {
        let current_calories = read_elf_backpack_calories(buffer, &mut offset).unwrap();
        if current_calories > max_calories {
            max_calories = current_calories;
        }
    }

    return max_calories.to_string()
}

const AMOUNT_MAX: usize = 3;

pub(crate) fn part2(input: &String) -> String {
    let buffer = input.as_bytes();
    let mut offset = 0;

    let mut max_calories = [0; AMOUNT_MAX];
    let mut current_calories;
    let mut cursor = 0;

    while offset < input.len() {
        current_calories = read_elf_backpack_calories(buffer, &mut offset).unwrap();
        if current_calories > max_calories[AMOUNT_MAX - 1] {
            while cursor < AMOUNT_MAX {
                if current_calories > max_calories[cursor] {
                    let previous_max = max_calories[cursor];
                    max_calories[cursor] = current_calories;
                    current_calories = previous_max;
                }
                cursor += 1;
            }
            cursor = 0;
        }
    }

    let mut total_calories = 0;
    for calories in max_calories {
        total_calories += calories;
    }

    return total_calories.to_string()
}