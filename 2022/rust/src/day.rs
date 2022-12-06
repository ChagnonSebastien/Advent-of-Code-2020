use std::fs::read_to_string;
use std::time::SystemTime;

pub(crate) struct Solutions<'a> {
    pub(crate) part2: Vec<&'a dyn Fn(&String) -> String>,
    pub(crate) part1: Vec<&'a dyn Fn(&String) -> String>,
}

pub(crate) struct Day<'a> {
    pub(crate) day: usize,
    pub(crate) solutions: Option<Solutions<'a>>,
}

impl Day<'_> {
    pub(crate) fn benchmark(&self, samples: usize) -> u128 {
        let input = read_to_string(format!("input/{:02}", self.day)).expect("Input file could not be read.");
        let mut answer = String::new();

        let start = SystemTime::now();
        for _ in 0..samples {
            answer = self.solutions.as_ref().unwrap().part1.get(0).unwrap()(&input);
        }
        let part1_duration = SystemTime::now().duration_since(start).unwrap().as_micros() / samples as u128;
        println!("Part 1: {answer}");
        println!("Took an average of {}μs over {} samples", part1_duration, samples);

        let start = SystemTime::now();
        for _ in 0..samples {
            answer = self.solutions.as_ref().unwrap().part2.get(0).unwrap()(&input);
        }
        let part2_duration = SystemTime::now().duration_since(start).unwrap().as_micros() / samples as u128;
        println!("Part 2: {answer}");
        println!("Took an average of {}μs over {} samples", part2_duration, samples);

        return part1_duration + part2_duration;
    }
}