use crate::utils::Part;
use std::collections::{BinaryHeap, HashSet};
use std::cmp::Ordering;

#[derive(Copy, Clone)]
struct Path {
  x: i64,
  y: i64,
  moves: i64,
  x_goal: i64,
  y_goal: i64,
}

impl Path {
  fn unique_key(&self) -> u64 {
    (self.x as u64) << 32 | (self.y as u64)
  }

  fn neighbors(&self) -> Vec<Self> {
    let mut possibilities = Vec::new();
    for (x_offset, y_offset) in &[(0,1),(0,-1),(1,0),(-1,0)] {
      if &self.x + x_offset < 0 || &self.y + y_offset < 0 { continue }
      let mut neighbor = *self;
      neighbor.x += x_offset;
      neighbor.y += y_offset;
      neighbor.moves += 1;
      possibilities.push(neighbor);
    }
    possibilities
  }

  fn cost(&self) -> i64 {
    &self.moves + (&self.x_goal - &self.x).abs() + (&self.y_goal - &self.y).abs()
  }

  fn is_goal(&self) -> bool {
    (&self.x_goal - &self.x).abs() + (&self.y_goal - &self.y).abs() == 0
  }
}

impl Ord for Path {
  fn cmp(&self, other: &Self) -> Ordering {
    other.cost().cmp(&self.cost())
  }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        self.cost() == other.cost()
    }
}
impl Eq for Path {}

struct Office {
  secret: i64,
  known_positions: HashSet<u64>,
  paths: BinaryHeap<Path>,
}

impl Office {
  fn is_valid(&self, x: i64, y: i64) -> bool {
    let mut t = x*x + 3*x + 2*x*y + y + y*y;
    t += self.secret;
    let mut c = 0;
    while t > 0 {
      c += t & 1;
      t >>= 1;
    };
    c % 2 == 0
  }

  fn solve(&mut self, initial_position: &Path, part: &Part) -> i64 {
    self.known_positions.insert(initial_position.unique_key());
    
    let mut position = *initial_position;
    while match part {Part::PartOne => !position.is_goal(), Part::PartTwo => true} {
      for neighbor in position.neighbors() {
        if self.is_valid(neighbor.x, neighbor.y) {
          if *part == Part::PartOne || neighbor.moves <= 50 {
            if self.known_positions.insert(neighbor.unique_key()) {
              self.paths.push(neighbor);
            }
          }
        }
      }

      let potential = self.paths.pop();
      match potential {
        Some(p) => position = p,
        None => {
          match part {
            Part::PartOne => panic!("No more available paths"),
            Part::PartTwo =>  return self.known_positions.len() as i64,
          }
        },
      };
    };
    position.moves
  }
}

pub fn execute(input: String, part: &Part) {
  let boss_favorite_number = input.parse::<i64>().unwrap();
  let mut office = Office {
    secret: boss_favorite_number,
    known_positions: HashSet::new(),
    paths: BinaryHeap::new(),
  };
  let solution = office.solve(&Path {
    x: 1,
    y: 1,
    moves: 0,
    x_goal: 31,
    y_goal: 39,
  }, part);

  match part {
    Part::PartOne => println!("Puzzle can be solved in {} moves.", solution),
    Part::PartTwo => println!("{} available moves.", solution),
  };
}
