use crate::utils::Part;
use std::collections::HashMap;
use regex::Regex;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Copy, Clone)]
enum ContainerType {
  Robot(usize), Output(usize)
}

impl Display for ContainerType {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
      write!(f, "{}", match self {
        ContainerType::Robot(n) => format!("Robot({})", n),
        ContainerType::Output(n) => format!("Output({})", n),
      })
  }
}

#[derive(Copy, Clone)]
struct Robot {
  number: usize,
  items: [Option<usize>; 2],
  high_to: ContainerType,
  low_to: ContainerType,
}

impl Robot {
  fn receive_item(&mut self, item: usize) {
    if self.items[0].is_none() {
      self.items[0] = Some(item);
    } else if self.items[1].is_none() {
      self.items[1] = Some(item);
    } else {
      panic!("Robot does not have more than 2 hands");
    }
  }

  fn is_giving(&self) -> bool {
    self.items[0].is_some() && self.items[1].is_some()
  }

  fn give(&mut self, part: &Part) -> Option<[(ContainerType, usize); 2]> {
    if !self.is_giving() {
      panic!("Robot cannot give if he doesn't have 2 items.");
    }

    let a = self.items[0].unwrap();
    let b = self.items[1].unwrap();
    self.items[0] = None;
    self.items[1] = None;

    let high = match a > b { true => a, false => b };
    let low = match a > b { true => b, false => a };

    match *part == Part::PartOne && high == 61 && low == 17 {
      true => None,
      false => Some([
        (self.high_to, high),
        (self.low_to, low)
      ]),
    }
  }
}

pub fn execute(input: String, part: &Part) {
  let mut robots: HashMap<usize, Robot> = HashMap::new();
  let mut initial_state: Vec<(usize, usize)> = Vec::new();
  let re = Regex::new(r"(bot|output|value) [0-9]+").unwrap();

  for instruction in input.split("\n") {
    let parts = re.find_iter(instruction).map(|x| x.as_str()).collect::<Vec<&str>>();
    match parts.len() {
      2 => {
        let value = parts[0].rsplit(" ").next().unwrap().parse::<usize>().unwrap();
        let n = parts[1].rsplit(" ").next().unwrap().parse::<usize>().unwrap();
        initial_state.push((n, value));
      },
      3 => {
        let n = parts[0].rsplit(" ").next().unwrap().parse::<usize>().unwrap();
        let low_n = parts[1].rsplit(" ").next().unwrap().parse::<usize>().unwrap();
        let high_n = parts[2].rsplit(" ").next().unwrap().parse::<usize>().unwrap();

        robots.insert(n, Robot {
          number: n,
          items: [None; 2],
          low_to: match parts[1].contains("bot") {
            true => ContainerType::Robot(low_n),
            false => ContainerType::Output(low_n),
          },
          high_to: match parts[2].contains("bot") {
            true => ContainerType::Robot(high_n),
            false => ContainerType::Output(high_n),
          },
        });
      },
      _ => panic!("Invalid instruction: {}", instruction),
    }
  }

  for (to, value) in initial_state {
    let robot = robots.get_mut(&to).unwrap();
    robot.receive_item(value);
  }

  let mut outputs: HashMap<usize, usize> = HashMap::new();

  while {
    let potential_container = robots.iter_mut().find_map(|(_, b)| match b.is_giving() {
      true => Some(b),
      false => None,
    });
    match potential_container {
      Some(robot) => {
        println!("Robot({}) can give!", robot.number);
        match robot.give(part) {
          Some(gifts) => {
            for (to, value) in gifts.iter() {
              println!("Giving {} to {}", value, to);
              match to {
                ContainerType::Robot(n) => {
                  let giving_to = robots.get_mut(n).unwrap();
                  giving_to.receive_item(*value);
                },
                ContainerType::Output(n) => {
                  outputs.insert(*n, *value);
                },
              }
            }
            true
          },
          None => {
            println!("Found the robot comparing values 17 and 61!");
            false
          },
        }
      },
      None => false
    }
  } {}

  if *part == Part::PartTwo {
    println!();
    for (number, item) in &outputs {
      println!("Output {} contains items: {}", number, item);
    }

    let o0 = outputs.get(&0).unwrap();
    let o1 = outputs.get(&1).unwrap();
    let o2 = outputs.get(&2).unwrap();
    println!("\nOutput(0) * Output(1) * Output(2) = {}", o0 * o1 * o2);
  }


}
