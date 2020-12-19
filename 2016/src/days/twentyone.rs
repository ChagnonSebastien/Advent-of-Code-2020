use crate::utils::Part;

enum Direction {
  Left, Right
}


enum Instruction {
  SwapPosition(usize, usize),
  SwapLetter(char, char),
  RotateSteps(Direction, usize),
  RotateLetter(char),
  Reverse(usize, usize),
  Move(usize, usize),
}

impl Instruction {
  fn apply(&self, s: String) -> String {
    match self {
      Instruction::SwapPosition(a, b) => {
        let chars: Vec<char> = s.chars().collect();
        let mut new = String::new();
        for i in 0..chars.len() {
          if i == *a {
            new.push(chars[*b])
          } else if i == *b {
            new.push(chars[*a])
          } else {
            new.push(chars[i])
          }
        }
        new
      },
      Instruction::SwapLetter(a, b) => {
        let chars: Vec<char> = s.chars().collect();
        let mut new = String::new();
        for i in 0..chars.len() {
          if chars[i] == *a {
            new.push(*b)
          } else if chars[i] == *b {
            new.push(*a)
          } else {
            new.push(chars[i])
          }
        }
        new
      },
      Instruction::RotateSteps(d, a) => {
        let rotation_split = match d {
          Direction::Left => *a % s.len(),
          Direction::Right => s.len() - (*a % s.len()),
        };
        format!("{}{}", &s[rotation_split..], &s[..rotation_split])
      },
      Instruction::RotateLetter(c) => {
        let index = s.find(|x| *c == x).unwrap();
        let rotation_amount = 1 + index + match index >= 4 { true => 1, false => 0 };
        Instruction::RotateSteps(Direction::Right, rotation_amount).apply(s)
      },
      Instruction::Reverse(a, b) => {
        format!("{}{}{}", &s[..*a], &s[*a..*b+1].chars().rev().collect::<String>(), &s[*b+1..])
      },
      Instruction::Move(a, b) => {
        let mut chars: Vec<char> = s.chars().collect();
        let c = chars.remove(*a);
        chars.insert(*b, c);
        chars.iter().collect()
      },
    }
  }

  fn undo(&self, s: String) -> String {
    match self {
      Instruction::SwapPosition(a, b) => Instruction::SwapPosition(*a, *b).apply(s),
      Instruction::SwapLetter(a, b) => Instruction::SwapLetter(*a, *b).apply(s),
      Instruction::RotateSteps(d, a) => Instruction::RotateSteps(match d {
        Direction::Right => Direction::Left,
        Direction::Left => Direction::Right
      }, *a).apply(s),
      Instruction::RotateLetter(c) => {
        let index = s.find(|x| *c == x).unwrap();
        for i in 0..s.len() {
          let p = i + 1 + match i >= 4 { true => 1, false => 0 };
          if (index + 2*s.len() - p) % s.len() == i {
            return Instruction::RotateSteps(Direction::Left, p).apply(s)
          }
        }
        panic!("Rotation not found");
      },
      Instruction::Reverse(a, b) => Instruction::Reverse(*a, *b).apply(s),
      Instruction::Move(a, b) => {
        let mut chars: Vec<char> = s.chars().collect();
        let c = chars.remove(*b);
        chars.insert(*a, c);
        chars.iter().collect()
      },
    }
  }

  fn from_string(s: &str) -> Instruction {
    let parts: Vec<&str> = s.split(" ").collect();
    let verb = parts[0];
    match verb {
      "swap" => {
        let subject = parts[1];
        match subject {
          "position" => Instruction::SwapPosition(parts[2].parse().unwrap(), parts[5].parse().unwrap()),
          "letter" => Instruction::SwapLetter(parts[2].chars().next().unwrap(), parts[5].chars().next().unwrap()),
          _ => panic!("Subject not implemented"),
        }
      },
      "rotate" => {
        let subject = parts[1];
        match subject {
          "left" => Instruction::RotateSteps(Direction::Left, parts[2].parse().unwrap()),
          "right" => Instruction::RotateSteps(Direction::Right, parts[2].parse().unwrap()),
          "based" => Instruction::RotateLetter(parts[6].chars().next().unwrap()),
          _ => panic!("Subject not implemented"),
        }
      }
      "reverse" => Instruction::Reverse(parts[2].parse().unwrap(), parts[4].parse().unwrap()),
      "move" => Instruction::Move(parts[2].parse().unwrap(), parts[5].parse().unwrap()),
      _ => panic!("Verb not implemented"),
    }
  }
}


pub fn execute(input: String, part: &Part) {
  match part {
    Part::PartOne => {
      let mut code = String::from("abcdefgh");
      for line in input.split("\n") {
        code = Instruction::from_string(line).apply(code);
      }
      println!("New code: {}", code);
    },
    Part::PartTwo => {
      let mut reverse_engineered = String::from("fbgdceah");
      for line in input.split("\n").collect::<Vec<&str>>().iter().rev() {
        reverse_engineered = Instruction::from_string(line).undo(reverse_engineered);
      }
      println!("Reverse engineered code: {}", reverse_engineered);

    }
  }
}
