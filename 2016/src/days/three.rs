use crate::utils::Part;
use std::fmt::{Display, Formatter, Result as FmtResult};

struct Triangle {
  sides: (usize, usize, usize),
}

impl Triangle {
  fn from_strings(a: &str, b: &str, c: &str) -> Triangle {
    let side1 = a.trim().parse::<usize>().expect(format!("The string {} is not a valid side length. Expecting unsigned int.", a).as_str());
    let side2 = b.trim().parse::<usize>().expect(format!("The string {} is not a valid side length. Expecting unsigned int.", b).as_str());
    let side3 = c.trim().parse::<usize>().expect(format!("The string {} is not a valid side length. Expecting unsigned int.", c).as_str());
    Triangle { sides: (side1, side2, side3)}
  }

  fn is_valid(&self) -> bool {
    let (a, b, c) = self.sides;
    a + b > c && a + c > b && b + c > a
  }
}

impl Display for Triangle {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    let (a, b, c) = self.sides;
    write!(f, "{{ sides: ({}, {}, {}) }}", a, b, c)
  }
}

pub fn execute(input: String, part: &Part) {
  let lines = input.split("\n");
  let mut valid_amount = 0;

  match part {
    Part::PartOne => {
      for line in lines {
        let triangle = Triangle::from_strings(&line[0..5], &line[5..10], &line[10..15]);
        println!("New Triangle: {}", triangle);
        match triangle.is_valid() {
          true => {
            println!("Valid");
            valid_amount += 1;
          },
          false => println!("Error: This triangle is not valid!"),
        }
      }
    },
    Part::PartTwo => {
      let mut buffer: Vec<&str> = Vec::new();
      for line in lines {
        buffer.push(line);

        if buffer.len() == 3 {
          for i in 0..3 {
            let triangle = Triangle::from_strings(&buffer[0][5*i+0..5*i+5], &buffer[1][5*i+0..5*i+5], &buffer[2][5*i+0..5*i+5]);
            println!("New Triangle: {}", triangle);
            match triangle.is_valid() {
              true => {
                println!("Valid");
                valid_amount += 1;
              },
              false => println!("Error: This triangle is not valid!"),
            }
          }
          buffer.clear();
        }
      }
    },
  }

  println!("\nAmount of valid triangles: {}", valid_amount);
}
