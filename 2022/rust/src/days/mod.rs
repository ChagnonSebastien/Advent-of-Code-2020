mod day01;
mod day02;
mod day03;
mod day04;
mod day05;

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
            _ => None
        }
    }
}