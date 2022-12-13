use std::fs::read_to_string;
use std::time::SystemTime;

pub(crate) struct Solutions<'a> {
    pub(crate) part2: Vec<&'a dyn Fn(&[u8]) -> String>,
    pub(crate) part1: Vec<&'a dyn Fn(&[u8]) -> String>,
}

pub(crate) struct Day<'a> {
    pub(crate) day: usize,
    pub(crate) solutions: Option<Solutions<'a>>,
}

impl Day<'_> {
    fn run_part(&self, samples: usize, part: usize, buffer: &[u8], benchmark: Option<usize>) -> u128 {
        let mut answer = String::new();
        let start = SystemTime::now();
        for _ in 0..samples {
            answer = match part {
                1 => &self.solutions.as_ref().unwrap().part1,
                2 => &self.solutions.as_ref().unwrap().part2,
                _ => panic!(""),
            }.get(0).unwrap()(buffer);
        }
        let duration = SystemTime::now().duration_since(start).unwrap().as_micros() / samples as u128;
        println!("Part {part}: \x1b[1m\x1b[92m{answer}\x1b[0m");
        if benchmark.is_some() {
            println!("Took an average of {} μs", duration);
        }
        return duration;
    }

    pub(crate) fn run(&self, benchmark: Option<usize>) -> u128 {
        let input = read_to_string(format!("input/{:02}", self.day)).expect("Input file could not be read.");
        let samples = benchmark.unwrap_or(1);

        let part1_duration = self.run_part(samples, 1, input.as_bytes(), benchmark);
        let part2_duration = self.run_part(samples, 2, input.as_bytes(), benchmark);

        return part1_duration + part2_duration;
    }
}