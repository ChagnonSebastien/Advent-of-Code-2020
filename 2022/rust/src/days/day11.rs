use crate::array_utils::{product_n, top_n};
use crate::days::day11::Operation::{PLUS, TIMES};
use crate::parser::read_unsigned_int;

const MAX_HAND_SIZE: usize = 24;
const AMOUNT_MONKEYS: usize = 8;
const CHASING_AMOUNT: usize = 2;

enum Term {
    OLD, NUMBER(usize)
}

impl Term {
    fn parse(buffer: &[u8], offset: &mut usize) -> Self {
        match buffer[*offset] as char {
            'o' => {
                *offset += 3;
                Term::OLD
            },
            _ => Term::NUMBER(read_unsigned_int(buffer, offset).unwrap() as usize),
        }
    }

    fn value(&self, old: usize) -> usize {
        match self {
            Term::OLD => old,
            Term::NUMBER(value) => *value,
        }
    }
}

enum Operation {
    PLUS(Term, Term), TIMES(Term, Term)
}

impl Operation {
    fn parse(buffer: &[u8], offset: &mut usize) -> Self {
        let first_term = Term::parse(buffer, offset);
        *offset += 1;
        let operator = buffer[*offset] as char;
        *offset += 2;
        let second_term = Term::parse(buffer, offset);
        match operator {
            '*' => TIMES(first_term, second_term),
            '+' => PLUS(first_term, second_term),
            _ => panic!("Unknown operator: {}", operator),
        }
    }

    fn apply(&self, old: usize) -> usize {
        match self {
            PLUS(a, b) => a.value(old) + b.value(old),
            TIMES(a, b) => a.value(old) * b.value(old),
        }
    }
}

struct Monkey {
    inventory: [usize; MAX_HAND_SIZE],
    inventory_size: usize,
    operation: Operation,
    test: usize,
    test_outcome_true: usize,
    test_outcome_false: usize,
}

impl Default for Monkey {
    fn default() -> Self {
        Monkey {
            inventory: [0; MAX_HAND_SIZE],
            inventory_size: 0,
            operation: PLUS(Term::OLD, Term::OLD),
            test: 0,
            test_outcome_true: 0,
            test_outcome_false: 0,
        }
    }
}

impl Monkey {
    fn parse(&mut self, buffer: &[u8], offset: &mut usize) {
        *offset += 7;
        read_unsigned_int(buffer, offset).expect("Was expecting the monkey index");
        *offset += 18;
        while buffer[*offset] != '\n' as u8 {
            *offset += 2;
            self.inventory[self.inventory_size] = read_unsigned_int(buffer, offset).unwrap() as usize;
            self.inventory_size += 1;
        }
        *offset += 20;
        self.operation = Operation::parse(buffer, offset);
        *offset += 22;
        self.test = read_unsigned_int(buffer, offset).unwrap() as usize;
        *offset += 30;
        self.test_outcome_true = read_unsigned_int(buffer, offset).unwrap();
        *offset += 31;
        self.test_outcome_false = read_unsigned_int(buffer, offset).unwrap();
        *offset += 2;
    }

    fn consider_throw(&self, worry_level: usize) -> usize {
        match worry_level % self.test {
            0 => self.test_outcome_true,
            _ => self.test_outcome_false,
        }
    }
}

fn parse_monkeys(buffer: &[u8]) -> [Monkey; AMOUNT_MONKEYS] {
    let mut offset = 0;
    let mut monkeys: [Monkey; AMOUNT_MONKEYS] = Default::default();
    let mut monkey_index = 0;
    while offset < buffer.len() {
        monkeys[monkey_index].parse(buffer, &mut offset);
        monkey_index += 1;
    }
    return monkeys;
}

fn simulate(monkeys: &mut [Monkey; AMOUNT_MONKEYS], rounds: usize, tensed: bool) -> [usize; AMOUNT_MONKEYS] {

    let mut modulus = 1;
    if tensed {
        for i in 0..monkeys.len() {
            modulus *= monkeys[i].test;
        }
    }

    let mut total_inspections = [0 as usize; AMOUNT_MONKEYS];
    for _ in 0..rounds {
        for monkey_index in 0..AMOUNT_MONKEYS {
            for considering_item in 0..monkeys[monkey_index].inventory_size {
                let mut worry_level = monkeys[monkey_index].inventory[considering_item];
                worry_level = monkeys[monkey_index].operation.apply(worry_level);
                total_inspections[monkey_index] += 1;
                if tensed {
                    worry_level %= modulus;
                } else {
                    worry_level /= 3;
                }
                let throwing_to = monkeys[monkey_index].consider_throw(worry_level);
                monkeys[throwing_to].inventory[monkeys[throwing_to].inventory_size] = worry_level;
                monkeys[throwing_to].inventory_size += 1;
            }
            monkeys[monkey_index].inventory_size = 0;
        }
    }

    return total_inspections;
}

pub(crate) fn part1(buffer: &[u8]) -> String {
    let mut monkeys = parse_monkeys(buffer);
    let mut total_inspections = simulate(&mut monkeys, 20, false);
    top_n(&mut total_inspections, CHASING_AMOUNT);
    return product_n(&total_inspections, CHASING_AMOUNT).to_string()
}

pub(crate) fn part2(buffer: &[u8]) -> String {
    let mut monkeys = parse_monkeys(buffer);
    let mut total_inspections = simulate(&mut monkeys, 10000, true);
    top_n(&mut total_inspections, CHASING_AMOUNT);
    return product_n(&total_inspections, CHASING_AMOUNT).to_string()
}