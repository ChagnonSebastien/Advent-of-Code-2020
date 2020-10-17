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
