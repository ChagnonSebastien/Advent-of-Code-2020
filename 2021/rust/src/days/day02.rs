use crate::parser::{read_unsigned_int, skip_word};

enum Word {
    FORWARD, DOWN, UP
}

struct Instruction {
    word: Word,
    amount: usize
}

impl Instruction {
    fn parse(buffer: &[u8], cursor: &mut usize) -> Instruction {
        let word = match buffer[*cursor] as char {
            'f' => Word::FORWARD,
            'd' => Word::DOWN,
            'u' => Word::UP,
            _ => panic!("Unknown instruction")
        };
        skip_word(buffer, cursor);
        *cursor += 1;
        let amount = read_unsigned_int(buffer, cursor);
        *cursor += 1;
        Instruction { word, amount }
    }
}

pub(crate) fn part1(buffer: &[u8]) -> String {
    let mut cursor = 0;
    let mut position = 0;
    let mut depth = 0;

    while cursor < buffer.len() {
        let instruction = Instruction::parse(buffer, &mut cursor);
        match instruction.word {
            Word::FORWARD => {
                position += instruction.amount;
            }
            Word::DOWN => {
                depth += instruction.amount;
            }
            Word::UP => {
                depth -= instruction.amount;
            }
        }
    }

    return (position * depth).to_string()
}

pub(crate) fn part2(buffer: &[u8]) -> String {
    let mut cursor = 0;
    let mut position = 0;
    let mut aim = 0;
    let mut depth = 0;

    while cursor < buffer.len() {
        let instruction = Instruction::parse(buffer, &mut cursor);
        match instruction.word {
            Word::FORWARD => {
                position += instruction.amount;
                depth += aim * instruction.amount;
            }
            Word::DOWN => {
                aim += instruction.amount;
            }
            Word::UP => {
                aim -= instruction.amount;
            }
        }
    }

    return (position * depth).to_string()
}