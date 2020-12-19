use crate::utils::Part;
use std::collections::{LinkedList, VecDeque};

pub fn execute(input: String, part: &Part) {
  let amount_elves = input.parse::<usize>().unwrap();

  match part {
    Part::PartOne => {
      let mut elves_circle = LinkedList::<(usize, usize)>::new();
      for i in 0..amount_elves {
        elves_circle.push_back((i + 1, 1));
      }

      while elves_circle.len() > 1 {
        let (elf, current) = elves_circle.pop_front().unwrap();
        let (_, steal_amount) = elves_circle.pop_front().unwrap();
        elves_circle.push_back((elf, current + steal_amount));
      }

      println!("The last elf is no. {}", elves_circle.pop_front().unwrap().0);
    },
    Part::PartTwo => {
      let mut elves_circle = VecDeque::<usize>::new();
      for i in 0..amount_elves {
        elves_circle.push_back(i+1);
      }
      
      while elves_circle.len() > 1 {
        elves_circle.remove(elves_circle.len() / 2);
        let saved = elves_circle.pop_front().unwrap();
        elves_circle.push_back(saved);
      }
      
      println!("The last elf is no. {}", elves_circle.pop_front().unwrap());
    }
  }

}
