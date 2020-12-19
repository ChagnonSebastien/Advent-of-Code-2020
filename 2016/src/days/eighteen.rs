use crate::utils::Part;

pub fn execute(input: String, part: &Part) {
  let mut row = input.chars().map(|x| x == '^').collect::<Vec<bool>>();
  let mut traps_amount = 0;
  for _ in 0..match part { Part::PartOne => 40, Part::PartTwo => 400000 } {
    for t in &row {
      traps_amount += match t { true => 0, false => 1 };
    }
    let mut new_row = Vec::<bool>::new();
    for i in 0..row.len() {
      let a = match i == 0 { true => false, false => *row.get(i-1).unwrap() };
      let b = *row.get(i).unwrap();
      let c = match row.get(i+1) { Some(t) => *t, None => false };
      new_row.push((a&&b&&!c) || (!a&&b&&c) || (a&&!b&&!c) || (!a&&!b&&c))
    }
    row = new_row;
  }

  println!("Amount of traps: {}", traps_amount);
}
