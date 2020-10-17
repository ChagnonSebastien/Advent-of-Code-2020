pub mod one {
  use crate::utils::Part;
  use std::fmt::{Display, Formatter, Result as FmtResult};

  enum Direction {
    NORTH,
    SOUTH,
    EAST,
    WEST,
  }

  impl Display for Direction {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
      write!(f, "{}", match self {
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

  pub fn execute(input: String, part: &Part) {
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
}

pub mod two {
  use crate::utils::Part;
  use std::fmt::{Display, Formatter, Result as FmtResult};

  enum Direction { UP, LEFT, RIGHT, DOWN }

  impl Direction {
    fn from_char(character: &char) -> Result<Direction, String> {
      match character {
        'U' => Ok(Direction::UP),
        'D' => Ok(Direction::DOWN),
        'L' => Ok(Direction::LEFT),
        'R' => Ok(Direction::RIGHT),
        _ => Err(format!("{} is not a valid direction character.", character))
      }
    }
  }

  enum Key {
    ONE,    TWO,    THREE,
    FOUR,   FIVE,   SIX,
    SEVEN,  EIGHT,  NINE,
    A, B, C, D,
  }

  impl Display for Key {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
      write!(f, "{}", match self {
        Key::ONE => "1",
        Key::TWO => "2",
        Key::THREE => "3",
        Key::FOUR => "4",
        Key::FIVE => "5",
        Key::SIX => "6",
        Key::SEVEN => "7",
        Key::EIGHT => "8",
        Key::NINE => "9",
        Key::A => "A",
        Key::B => "B",
        Key::C => "C",
        Key::D => "D",
      })
    }
  }

  impl Key {
    fn neighbor(&self, direction: &Direction, part: &Part) -> Key {
      match self {
        Key::ONE => match direction {
          Direction::UP => match part { Part::PartOne => Key::ONE, Part::PartTwo => Key::ONE }
          Direction::RIGHT => match part { Part::PartOne => Key::TWO, Part::PartTwo => Key::ONE }
          Direction::DOWN => match part { Part::PartOne => Key::FOUR, Part::PartTwo => Key::THREE }
          Direction::LEFT => match part { Part::PartOne => Key::ONE, Part::PartTwo => Key::ONE }
        },
        Key::TWO => match direction {
          Direction::UP => match part { Part::PartOne => Key::TWO, Part::PartTwo => Key::TWO }
          Direction::RIGHT => match part { Part::PartOne => Key::THREE, Part::PartTwo => Key::THREE }
          Direction::DOWN => match part { Part::PartOne => Key::FIVE, Part::PartTwo => Key::SIX }
          Direction::LEFT => match part { Part::PartOne => Key::ONE, Part::PartTwo => Key::TWO }
        },
        Key::THREE => match direction {
          Direction::UP => match part { Part::PartOne => Key::THREE, Part::PartTwo => Key::ONE }
          Direction::RIGHT => match part { Part::PartOne => Key::THREE, Part::PartTwo => Key::FOUR }
          Direction::DOWN => match part { Part::PartOne => Key::SIX, Part::PartTwo => Key::SEVEN }
          Direction::LEFT => match part { Part::PartOne => Key::TWO, Part::PartTwo => Key::TWO }
        },
        Key::FOUR => match direction {
          Direction::UP => match part { Part::PartOne => Key::ONE, Part::PartTwo => Key::FOUR }
          Direction::RIGHT => match part { Part::PartOne => Key::FIVE, Part::PartTwo => Key::FOUR }
          Direction::DOWN => match part { Part::PartOne => Key::SEVEN, Part::PartTwo => Key::EIGHT }
          Direction::LEFT => match part { Part::PartOne => Key::FOUR, Part::PartTwo => Key::THREE }
        },
        Key::FIVE => match direction {
          Direction::UP => match part { Part::PartOne => Key::TWO, Part::PartTwo => Key::FIVE }
          Direction::RIGHT => match part { Part::PartOne => Key::SIX, Part::PartTwo => Key::SIX }
          Direction::DOWN => match part { Part::PartOne => Key::EIGHT, Part::PartTwo => Key::FIVE }
          Direction::LEFT => match part { Part::PartOne => Key::FOUR, Part::PartTwo => Key::FIVE }
        },
        Key::SIX => match direction {
          Direction::UP => match part { Part::PartOne => Key::THREE, Part::PartTwo => Key::TWO }
          Direction::RIGHT => match part { Part::PartOne => Key::SIX, Part::PartTwo => Key::SEVEN }
          Direction::DOWN => match part { Part::PartOne => Key::NINE, Part::PartTwo => Key::A }
          Direction::LEFT => match part { Part::PartOne => Key::FIVE, Part::PartTwo => Key::FIVE }
        },
        Key::SEVEN => match direction {
          Direction::UP => match part { Part::PartOne => Key::FOUR, Part::PartTwo => Key::THREE }
          Direction::RIGHT => match part { Part::PartOne => Key::EIGHT, Part::PartTwo => Key::EIGHT }
          Direction::DOWN => match part { Part::PartOne => Key::SEVEN, Part::PartTwo => Key::B }
          Direction::LEFT => match part { Part::PartOne => Key::SEVEN, Part::PartTwo => Key::SIX }
        },
        Key::EIGHT => match direction {
          Direction::UP => match part { Part::PartOne => Key::FIVE, Part::PartTwo => Key::FOUR }
          Direction::RIGHT => match part { Part::PartOne => Key::NINE, Part::PartTwo => Key::NINE }
          Direction::DOWN => match part { Part::PartOne => Key::EIGHT, Part::PartTwo => Key::C }
          Direction::LEFT => match part { Part::PartOne => Key::SEVEN, Part::PartTwo => Key::SEVEN }
        },
        Key::NINE => match direction {
          Direction::UP => match part { Part::PartOne => Key::SIX, Part::PartTwo => Key::NINE }
          Direction::RIGHT => match part { Part::PartOne => Key::NINE, Part::PartTwo => Key::NINE }
          Direction::DOWN => match part { Part::PartOne => Key::NINE, Part::PartTwo => Key::NINE }
          Direction::LEFT => match part { Part::PartOne => Key::EIGHT, Part::PartTwo => Key::EIGHT }
        },
        Key::A => match direction {
          Direction::UP => Key::SIX,
          Direction::RIGHT => Key::B,
          Direction::DOWN => Key::A,
          Direction::LEFT => Key::A,
        },
        Key::B => match direction {
          Direction::UP => Key::SEVEN,
          Direction::RIGHT => Key::C,
          Direction::DOWN => Key::D,
          Direction::LEFT => Key::A,
        },
        Key::C => match direction {
          Direction::UP => Key::EIGHT,
          Direction::RIGHT => Key::C,
          Direction::DOWN => Key::C,
          Direction::LEFT => Key::B,
        },
        Key::D => match direction {
          Direction::UP => Key::B,
          Direction::RIGHT => Key::D,
          Direction::DOWN => Key::D,
          Direction::LEFT => Key::D,
        },
      }
    }
  }

  fn key_from_instruction(line: &str, part: &Part) -> Key {
    let mut key = Key::FIVE;
    for character in line.chars() {
      let direction = Direction::from_char(&character).expect("dsa");
      key = key.neighbor(&direction, part);
    }
    key
  }

  pub fn execute(input: String, part: &Part) {
    let mut code = String::new();
    for line in input.split("\n") {
      println!("New Pattern: {}", line);
      let key = key_from_instruction(line, part);
      println!("Secret Key: {}\n", key);
      code.push_str(format!("{}", key).as_str());
    }
    println!("\nSecret Code: {}", code);
  }
}