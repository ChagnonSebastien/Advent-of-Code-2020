mod day01;
mod day02;
mod day03;

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
            _ => None
        }
    }
}