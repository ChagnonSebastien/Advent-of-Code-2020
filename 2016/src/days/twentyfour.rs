use crate::utils::Part;
use std::collections::{HashMap, BinaryHeap, HashSet};
use std::cmp::Ordering;
use std::usize;

#[derive(Copy, Clone)]
struct Position {
  x: usize, y: usize,
}

impl Position {
  fn key(&self) -> u16 {
    let a = self.x as u16;
    let b = self.y as u16;
    a | b << 8
  }

  fn neighbors(&self) -> Vec<Self> {
    vec!(
      Position {x: self.x + 1, y: self.y},
      Position {x: self.x - 1, y: self.y},
      Position {x: self.x, y: self.y + 1},
      Position {x: self.x, y: self.y - 1}
    )
  }
}

#[derive(Copy, Clone)]
struct Path {
  position: Position,
  goal: Position,
  moves: usize,
}

impl Path {
  fn neighbors(&self) -> Vec<Self> {
    self.position.neighbors().iter().map(|p| Path { position: *p, moves: self.moves + 1, goal: self.goal}).collect()
  }

  fn cost(&self) -> usize {
    self.moves + ((self.position.x as isize - self.goal.x as isize).abs() + (self.position.y as isize - self.goal.y as isize).abs()) as usize
  }

  fn key(&self) -> u16 {
    self.position.key()
  }

  fn is_solution(&self) -> bool {
    self.position.x == self.goal.x && self.position.y == self.goal.y
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

struct PipeNetwork {
  pipes: Vec<Vec<bool>>,
  locations: HashMap<char, Position>,
}

impl PipeNetwork {
  fn from_input(input: String) -> Self {
    let mut network = PipeNetwork {
      pipes: Vec::new(),
      locations: HashMap::new(),
    };

    let lines: Vec<&str> = input.split("\n").collect();
    for i in 0..lines.len() {
      network.pipes.push(Vec::new());
      let characters: Vec<char> = lines[i].chars().collect();
      for j in 0..characters.len() {
        let character = characters[j];
        match character {
          '#' => network.pipes[i].push(false),
          _ => network.pipes[i].push(true),
        }
        if character.is_numeric() {
          network.locations.insert(character, Position{ x: i, y: j });
        }
      }
    }
    network
  }

  fn shortest_path(&self, from: Position, to: Position) -> usize {
    let mut visited = HashSet::<u16>::new();
    let mut potentials = BinaryHeap::<Path>::new();
    
    let mut path = Path { goal: to, position: from, moves: 0 };
    visited.insert(path.key());

    while !path.is_solution() {
      for neighbor in path.neighbors() {
        if self.pipes[neighbor.position.x][neighbor.position.y] {
          if visited.insert(neighbor.key()) {
            potentials.push(neighbor);
          }
        }
      }
      path = potentials.pop().expect("No more paths");
    }

    path.moves
  }
}

fn permutation(prefix: String, s: String) -> Vec<String> {
  let mut p = Vec::new();
  let n = s.len();
  if n == 0 {
    p.push(prefix);
  } else {
    for i in 0..n {
      let mut chars = s.chars().collect::<Vec<char>>();
      let removed = chars.remove(i);
      let new = format!("{}{}", prefix, removed);
      p.append(&mut permutation(new, chars.iter().collect::<String>()));
    }
  }
  p
}

fn permutations(s: String) -> Vec<String> {
  permutation(String::new(), s)
}

pub fn execute(input: String, part: &Part) {
  let network = PipeNetwork::from_input(input);

  let keys: Vec<char> = network.locations.keys().map(|c| *c).collect();
  let mut distances = HashMap::<(char, char), usize>::new();
  for i in 0..network.locations.len() {
    for j in 0..network.locations.len() {
      let from = network.locations.get(&keys[i]).unwrap();
      let to = network.locations.get(&keys[j]).unwrap();
      distances.insert((keys[i], keys[j]), network.shortest_path(*from, *to));
    }
  }

  let in_game_keys: String = keys.iter().filter_map(|c| {
    match c {
      '0' => None,
      i => Some(*i),
    }
  }).collect();

  let mut shortest_distance: usize = !0;
  for p in permutations(in_game_keys) {
    let (end, mut distance) = p.chars().fold(('0', 0), |(a, d), n| (n, d + distances.get(&(a, n)).unwrap()));

    if *part == Part::PartTwo {
      distance += distances.get(&(end, '0')).unwrap();
    }

    if distance < shortest_distance {
      shortest_distance = distance;
    }
  }

  println!("Shortest distance: {}", shortest_distance);
}
