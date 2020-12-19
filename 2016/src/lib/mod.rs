use std::fmt::{Display, Formatter, Result as FmtResult};

pub enum CardinalPoint {
  NORTH,
  SOUTH,
  EAST,
  WEST,
}

impl Display for CardinalPoint {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    write!(f, "{}", match self {
      CardinalPoint::NORTH => "NORTH",
      CardinalPoint::WEST => "WEST",
      CardinalPoint::SOUTH => "SOUTH",
      CardinalPoint::EAST => "EAST",
    })
  }
}

#[derive(Clone)]
pub struct Vector2D {
  pub x: isize,
  pub y: isize,
}

impl Vector2D {
  pub fn manhattan_distance(&self, other: &Self) -> isize {
    (self.x - other.x).abs() + (self.y - other.y).abs()
  }
}

impl Display for Vector2D {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    write!(f, "({}, {})", self.x, self.y)
  }
}