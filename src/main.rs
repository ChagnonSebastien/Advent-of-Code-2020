use std::env;
use std::fs;
use std::fmt::{Display, Formatter, Result as FmtResult};

enum Part {
  PartOne,
  PartTwo,
}

enum Direction {
  NORTH,
  SOUTH,
  EAST,
  WEST,
}

impl Display for Direction {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    write!(f, "{:?}", match self {
      Direction::NORTH => "NORTH",
      Direction::WEST => "WEST",
      Direction::SOUTH => "SOUTH",
      Direction::EAST => "EAST",
    })
  }
}

#[derive(Clone)]
struct Vector2D {
  x: isize,
  y: isize,
}

impl Vector2D {
  fn manhattan_distance(&self, other: &Vector2D) -> isize {
    (self.x - other.x).abs() + (self.y - other.y).abs()
  }
}

impl Display for Vector2D {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    write!(f, "({}, {})", self.x, self.y)
  }
}

struct Santa {
  position: Vector2D,
  facing: Direction,
  history: Vec<Vector2D>,
}

impl Santa {
  fn has_been(&self, compare: &Vector2D) -> bool {
    for coord in &self.history {
      if coord.x == compare.x && coord.y == compare.y {
        return true
      }
    };
    false
  }

  fn rotate(&mut self, direction: &str) -> &Direction {
    match direction {
      "L" => {
        self.facing = match self.facing {
          Direction::NORTH => Direction::WEST,
          Direction::WEST => Direction::SOUTH,
          Direction::SOUTH => Direction::EAST,
          Direction::EAST => Direction::NORTH,
        };
      },
      "R" => {
        self.facing = match self.facing {
          Direction::NORTH => Direction::EAST,
          Direction::WEST => Direction::NORTH,
          Direction::SOUTH => Direction::WEST,
          Direction::EAST => Direction::SOUTH,
        };
      },
      _ => panic!("Unknown turn direction: {}", direction),
    };
    &self.facing
  }

  fn walk(&mut self, distance: isize, stop_on_visited: bool) -> Result<&Vector2D, &Vector2D> {
    for _ in 0..distance {
      match self.facing {
        Direction::NORTH => self.position.y -= 1,
        Direction::WEST => self.position.x -= 1,
        Direction::SOUTH => self.position.y += 1,
        Direction::EAST => self.position.x += 1,
      };
      if stop_on_visited && self.has_been(&self.position){
        return Err(&self.position)
      }
      self.history.push(self.position.clone());
    }
    Ok(&self.position)
  }
  
  fn follow_instruction(&mut self, parts: (&str, &str), stop_on_visited: bool) -> Result<&Vector2D, &Vector2D> {
    self.rotate(parts.0);
    self.walk(parts.1.parse::<isize>().unwrap(), stop_on_visited)
  }
}

impl Display for Santa {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    write!(f, "Santa( position: {}, facing: {} )", self.position, self.facing)
  }
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let raw_part = args.get(1).expect("You need to specify the part to run").as_str();
  let part = match raw_part {
    "1" => Part::PartOne,
    "2" => Part::PartTwo,
    _ => panic!("Invalid part {}", raw_part),
  };

  let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
  let input = String::from(contents.trim_end());

  day_one(input, part);
}

fn day_one(input: String, part: Part) {
  let stop_on_visited = match part {
    Part::PartOne => false,
    Part::PartTwo => true,
  };

  let initial_position = Vector2D { x: 0, y: 0 };
  let mut santa = Santa {
    position: initial_position.clone(),
    facing: Direction::NORTH,
    history: vec![initial_position.clone()],
  };
  println!("Initial position: {}", initial_position);

  for instruction in input.split(", ") {
    let string_instruction = String::from(instruction);
    let parts = string_instruction.split_at(1);
    println!("Instruction: Turn {} and move {} units", parts.0, parts.1);
    match santa.follow_instruction(parts, stop_on_visited) {
      Ok(new_position) => println!("New position: {}", new_position),
      Err(new_position) => {
        println!("Stopping because of crossing paths: {}", new_position);
        break;
      },
    }
  }

  println!("Distance from start: {} units", santa.position.manhattan_distance(&initial_position));
}
