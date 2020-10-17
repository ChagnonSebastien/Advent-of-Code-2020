use crate::utils::Part;
use crate::lib::{CardinalPoint, Vector2D};
use std::fmt::{Display, Formatter, Result as FmtResult};

struct Santa {
  position: Vector2D,
  facing: CardinalPoint,
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

  fn rotate(&mut self, direction: &str) -> &CardinalPoint {
    match direction {
      "L" => {
        self.facing = match self.facing {
          CardinalPoint::NORTH => CardinalPoint::WEST,
          CardinalPoint::WEST => CardinalPoint::SOUTH,
          CardinalPoint::SOUTH => CardinalPoint::EAST,
          CardinalPoint::EAST => CardinalPoint::NORTH,
        };
      },
      "R" => {
        self.facing = match self.facing {
          CardinalPoint::NORTH => CardinalPoint::EAST,
          CardinalPoint::WEST => CardinalPoint::NORTH,
          CardinalPoint::SOUTH => CardinalPoint::WEST,
          CardinalPoint::EAST => CardinalPoint::SOUTH,
        };
      },
      _ => panic!("Unknown turn direction: {}", direction),
    };
    &self.facing
  }

  fn walk(&mut self, distance: isize, stop_on_visited: bool) -> Result<&Vector2D, &Vector2D> {
    for _ in 0..distance {
      match self.facing {
        CardinalPoint::NORTH => self.position.y -= 1,
        CardinalPoint::WEST => self.position.x -= 1,
        CardinalPoint::SOUTH => self.position.y += 1,
        CardinalPoint::EAST => self.position.x += 1,
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

pub fn execute(input: String, part: &Part) {
  let stop_on_visited = match part {
    Part::PartOne => false,
    Part::PartTwo => true,
  };

  let initial_position = Vector2D { x: 0, y: 0 };
  let mut santa = Santa {
    position: initial_position.clone(),
    facing: CardinalPoint::NORTH,
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