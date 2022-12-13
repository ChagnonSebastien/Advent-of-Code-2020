mod day01;

use crate::day::{Day, Solutions};

pub(crate) fn get_day<'a>(day: usize) -> Day<'a> {
    Day {
        day,
        solutions: match day {
            1 => Some(Solutions {
                part1: vec![&day01::part1],
                part2: vec![&day01::part2],
            }),
            _ => None
        }
    }
}