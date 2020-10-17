mod utils;
mod days;
mod lib;

use std::env::args;
use std::fs::read_to_string;
use utils::{Part, Day};


fn main() {
  let args: Vec<String> = args().collect();
  
  let raw_day = args.get(1).expect("You need to specify the day to run").as_str();
  let day = Day::from_number(raw_day).expect("Invalid day.");
  
  let raw_part = args.get(2).expect("You need to specify the part to run").as_str();
  let part = Part::from_number(raw_part).expect("Invalid part.");

  let file_name = format!("inputs/input-{}.txt", day);
  let file = file_name.as_str();
  let contents = read_to_string(file).expect("Something went wrong reading the file");
  let input = String::from(contents.trim_end());

  match day {
    Day::DayOne => days::one::execute(input, &part),
    Day::DayTwo => days::two::execute(input, &part),
    Day::DayThree => days::three::execute(input, &part),
    Day::DayFour => days::four::execute(input, &part),
    _ => panic!("Day not yet implemented"),
  }
}
