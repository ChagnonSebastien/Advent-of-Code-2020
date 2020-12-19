use crate::utils::Part;
use regex::Regex;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::cmp::Ordering;
use std::collections::{HashSet, BinaryHeap};

struct CPU {
  x: usize, y: usize,
  size: usize, used: usize
}

impl CPU {
  fn free(&self) -> usize {
    self.size - self.used
  }

  fn from_string(s: &str) -> Self {
    let re = Regex::new(r"[0-9]+").unwrap();
    let nos: Vec<usize> = re.find_iter(s).map(|n| n.as_str().parse().unwrap()).collect();
    CPU {
      x: nos[0], y: nos[1],
      size: nos[2], used: nos[3]
    }
  }
}

impl Display for CPU {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    write!(f, "CPU({}, {}): {}/{}T", self.x, self.y, self.used, self.size)
  }
}

#[derive(Copy, Clone)]
struct Lab {
  cpus: [[bool; 29]; 35],
  x: u8, y: u8,
  gx: u8, gy: u8,
  moves: u16,
}

impl Lab {
  fn from_cpus(cpus: Vec<CPU>) -> Self {
    let smallest_sized_disk = cpus.iter().fold(10000, |smallest, cpu| match cpu.size < smallest { true => cpu.size, false => smallest });
    let zero = cpus.iter().find_map(|cpu| match cpu.used == 0 { true => Some((cpu.x, cpu.y)), false => None }).unwrap();
    let mut lab = Lab {
      cpus: [[true; 29]; 35],
      x: zero.0 as u8, y: zero.1 as u8,
      gx: 34, gy: 0,
      moves: 0,
    };

    for block in cpus.iter().filter(|cpu| cpu.used > smallest_sized_disk) {
      lab.cpus[block.x][block.y] = false;
    }

    lab
  }

  fn key(&self) -> u32 {
    let a = self.x as u32;
    let b = (self.y as u32) << 8;
    let c = (self.gx as u32) << 16;
    let d = (self.gy as u32) << 24;
    a|b|c|d
  }

  fn cost(&self) -> u16 {
    self.moves + self.gx as u16 + self.gy as u16
  }

  fn is_solution(&self) -> bool {
    self.gx == 0 && self.gy == 0
  }

  fn neighbors(&self) -> Vec<Self> {
    let mut neighbors = Vec::new();
    for (i, j) in &[(0,1),(0,-1),(1,0),(-1,0)] {
      if self.x == 0 && *i == -1 || self.x == 34 && *i == 1 || self.y == 0 && *j == -1 || self.y == 28 && *j == 1 { continue };
      let newx = (self.x as i8 + i) as u8;
      let newy = (self.y as i8 + j) as u8;

      if !self.cpus[newx as usize][newy as usize] { continue }

      let mut neighbor = *self;
      neighbor.moves += 1;
      neighbor.x = newx;
      neighbor.y = newy;
      if newx == self.gx && newy == self.gy {
        neighbor.gx = self.x;
        neighbor.gy = self.y;
      }
      neighbors.push(neighbor);
    }
    neighbors
  }
}

impl Ord for Lab {
  fn cmp(&self, other: &Self) -> Ordering {
    other.cost().cmp(&self.cost())
  }
}

impl PartialOrd for Lab {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Lab {
    fn eq(&self, other: &Self) -> bool {
        self.cost() == other.cost()
    }
}
impl Eq for Lab {}

impl Display for Lab {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    let mut s = String::new();

    for j in 0..29 {
      for i in 0..35 {
        if i == self.x.into() && j == self.y.into() {
          s.push('_');
        } else if i == self.gx.into() && j == self.gy.into() {
          s.push('G');
        } else if i == 0 && j == 0 {
          s.push('o');
        } else {
          s.push(match self.cpus[i][j] { true => '.', false => '#' });
        }
      }
      s.push_str("\n")
    }

    write!(f, "{}", s)
  }
}

pub fn execute(input: String, part: &Part) {
  let re = Regex::new(r"/dev/grid").unwrap();
  let cpus: Vec<CPU> = input
    .split("\n")
    .filter_map(|s| match re.is_match(s) { true => Some(s), false => None })
    .collect::<Vec<&str>>()
    .iter()
    .map(|s| CPU::from_string(s)).collect();

  match part {
    Part::PartOne => {
      let mut fits = 0;
      for i in 0..cpus.len() {
        for j in 0..cpus.len() {
          if i == j { continue }
          if cpus[i].used > 0 && cpus[i].used <= cpus[j].free() {
            println!("{} fits in {}", cpus[i], cpus[j]);
            fits += 1;
          }
        }
      }
      println!("There are {} viable pairs.", fits);
    },
    Part::PartTwo => {
      let mut visited = HashSet::<u32>::new();
      let mut potentials = BinaryHeap::<Lab>::new();
      
      let mut lab = Lab::from_cpus(cpus);
      visited.insert(lab.key());
    
      while !lab.is_solution() {
        for p in lab.neighbors() {
          if visited.insert(p.key()) {
            potentials.push(p);
          }
        }

        lab = potentials.pop().expect("No more potential labs");
      }

      println!("Found solution in {} moves", lab.moves);
    }
  }
}
