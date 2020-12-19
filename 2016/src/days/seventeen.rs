use crate::utils::Part;
use std::collections::{BinaryHeap};
use std::cmp::Ordering;
use md5;

struct Path {
  instructions: String,
  x: i8, y: i8,
}

impl Path {
  fn is_at_exit(&self) -> bool {
    self.x == 3 && self.y == 3
  }

  fn cost(&self) -> usize {
    self.instructions.len()
  }

  fn neighbors(&self) -> BinaryHeap<Self> {
    let digest = md5::compute(format!("{}", self.instructions));
    let hash = format!("{:x}", digest).chars().collect::<Vec<char>>();
    let mut neighbors = BinaryHeap::<Self>::new();
    for (c, (ox, oy), i) in &[('U',(0,-1),0),('D',(0,1),1),('L',(-1,0),2),('R',(1,0),3)] {
      if self.x + ox < 0 || self.x + ox > 3 || self.y + oy < 0 || self.y + oy > 3 { continue }
      let k = hash[*i];
      let open = k=='b'||k=='c'||k=='d'||k=='e'||k=='f';
      if open {
        neighbors.push(Path {
          instructions: format!("{}{}", self.instructions, c),
          x: self.x + ox, y: self.y + oy,
        })
      }
    }
    neighbors
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

pub fn part_one(input: String) {
  let mut paths = BinaryHeap::<Path>::new();
  let mut path = Path {
    instructions: String::from(&input),
    x: 0, y: 0,
  };

  while !path.is_at_exit() {
    paths.append(&mut path.neighbors());
    path = paths.pop().expect("No more paths");
  }

  println!("Path: {}", &path.instructions[input.len()..]);
}

pub fn part_two(input: String) {
  let mut paths = BinaryHeap::<Path>::new();
  let mut longest_path = 0;
  paths.push(Path {
    instructions: String::from(&input),
    x: 0, y: 0,
  });

  while paths.len() > 0 {
    let path = paths.pop().expect("No more paths");

    if path.is_at_exit(){
      if path.instructions.len() - input.len() > longest_path {
        longest_path = path.instructions.len() - input.len();
        println!("New longest path: {}, Remaining: {}", longest_path, paths.len());
      }
    } else {
      paths.append(&mut path.neighbors());
    }
  }

  println!("Longest path: {}", longest_path);
}

pub fn execute(input: String, part: &Part) {
  match part {
    Part::PartOne => part_one(input),
    Part::PartTwo => part_two(input),
  }
}
