mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;

use crate::day::{Day, Solutions};

pub(crate) fn get_day<'a>(day: usize) -> Day<'a> {
    Day {
        day,
        solutions: match day {
            1 => Some(Solutions {
                part1: vec![&day01::part1],
                part2: vec![&day01::part2],
            }),
            2 => Some(Solutions {
                part1: vec![&day02::part1],
                part2: vec![&day02::part2],
            }),
            3 => Some(Solutions {
                part1: vec![&day03::part1],
                part2: vec![&day03::part2],
            }),
            4 => Some(Solutions {
                part1: vec![&day04::part1],
                part2: vec![&day04::part2],
            }),
            5 => Some(Solutions {
                part1: vec![&day05::part1_stack, &day05::part1_memory],
                part2: vec![&day05::part2_stack, &day05::part2_memory],
            }),
            6 => Some(Solutions {
                part1: vec![&day06::part1],
                part2: vec![&day06::part2],
            }),
            7 => Some(Solutions {
                part1: vec![&day07::part1],
                part2: vec![&day07::part2],
            }),
            8 => Some(Solutions {
                part1: vec![&day08::part1, &day08::part1_unfinished_stack_forest_surveyor, &day08::part1_unfinished_mem_forest_surveyor],
                part2: vec![&day08::part2, &day08::part2_surveyor],
            }),
            9 => Some(Solutions {
                part1: vec![&day09::part1],
                part2: vec![&day09::part2],
            }),
            10 => Some(Solutions {
                part1: vec![&day10::part1],
                part2: vec![&day10::part2],
            }),
            11 => Some(Solutions {
                part1: vec![&day11::part1, &day11::part1_old],
                part2: vec![&day11::part2, &day11::part2_old, &day11::part2_oldest],
            }),
            12 => Some(Solutions {
                part1: vec![&day12::part1, &day12::part1_old],
                part2: vec![&day12::part2, &day12::part2_old],
            }),
            13 => Some(Solutions {
                part1: vec![&day13::part1],
                part2: vec![&day13::part2, &day13::part2_old],
            }),
            14 => Some(Solutions {
                part1: vec![&day14::part1, &day14::part1_simple, &day14::part1_complex_hashset],
                part2: vec![&day14::part2, &day14::part2_simple, &day14::part2_complex_hashset],
            }),
            15 => Some(Solutions {
                part1: vec![&day15::part1],
                part2: vec![&day15::part2],
            }),
            _ => None
        }
    }
}