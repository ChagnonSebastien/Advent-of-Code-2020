use crate::utils::Part;
use std::char;

pub fn execute(input: String, part: &Part) {
  let mut amounts: [[usize; 26]; 8] = [[0; 26]; 8];
  for word in input.split("\n") {
    for i in 0..word.len() {
      amounts[i][(word.chars().nth(i).unwrap() as usize) - 97] += 1;
    }
  }

  let secret = amounts.iter().map(|i| {
    let best_amount = i.iter().fold(match part {
      Part::PartOne => 0,
      Part::PartTwo => input.len(),
    }, |best, amount| if *amount > best {
      match part {
        Part::PartOne => *amount,
        Part::PartTwo => best,
      }
    } else {
      match part {
        Part::PartOne => best,
        Part::PartTwo => *amount,
      }
    });
    let best_index = i.iter().position(|i| *i == best_amount).unwrap();
    char::from_u32((best_index + 97) as u32).unwrap_or('#')
  }).collect::<String>();

  println!("{}", secret);
}
