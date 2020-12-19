use crate::utils::Part;
use std::collections::{HashMap, BinaryHeap};
use std::cmp::{Ord, Ordering, };
use std::char;

#[derive(Eq)]
struct KeyValuePair {
  character: char,
  amount: usize,
}

impl Ord for KeyValuePair {
  fn cmp(&self, other: &Self) -> Ordering {
    match self.amount.cmp(&other.amount) {
      Ordering::Greater => Ordering::Greater,
      Ordering::Less => Ordering::Less,
      Ordering::Equal => {
        match other.character.cmp(&self.character) {
          Ordering::Greater => Ordering::Greater,
          Ordering::Less => Ordering::Less,
          Ordering::Equal => Ordering::Equal,
        }
      },
    }
  }
}

impl PartialOrd for KeyValuePair {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl PartialEq for KeyValuePair {
  fn eq(&self, other: &Self) -> bool {
    self.amount == other.amount && self.character == other.character
  }
}

pub fn execute(input: String, part: &Part) {
  let mut sum_ids = 0;
  let mut valid_rooms: Vec<(&str, usize)> = Vec::new();

  for line in input.split("\n") {
    println!("{}", line);

    let name_limit = line.rfind('-')
      .expect(format!("Error: Encrypted room has the wrong format : {}", line).as_str());
    let room_name = &line[0..name_limit];

    let room_limit = line.rfind('[')
      .expect(format!("Error: Encrypted room has the wrong format : {}", line).as_str());
    let room_number = line[name_limit+1..room_limit].parse::<usize>()
      .expect(format!("Error: Wrong sector ID format : {}. Expected number.", &line[name_limit+1..room_limit]).as_str());
    
    let checksum = &line[room_limit+1..line.len()-1];

    let mut char_distr: HashMap<char, usize> = HashMap::new();
    for character in room_name.chars() {
      if character.eq(&'-') { continue }

      let prev_value = char_distr.get(&character);
      let new_value = prev_value.unwrap_or(&0) + 1;
      char_distr.insert(character, new_value);
    }

    let mut ordered_heap: BinaryHeap<KeyValuePair> = BinaryHeap::new();
    for (key, value) in char_distr.iter() {
      ordered_heap.push(KeyValuePair { character: *key, amount: *value })
    }

    let mut calculated_chacksum = String::new();
    for _ in 0..5 {
      calculated_chacksum.push(ordered_heap.pop().unwrap().character);
    }
    println!("Calculated Chacksum: {}", calculated_chacksum);

    if calculated_chacksum.eq(checksum) {
      sum_ids += room_number;
      valid_rooms.push((room_name, room_number));
    }
  }
  
  match part {
    Part::PartOne => println!("\nSum of valid room IDs: {}", sum_ids),
    Part::PartTwo => {
      println!("\nStarting room names validation");
      for (room_name, room_number) in valid_rooms {
        println!("Room: {} - {}", room_number, room_name);
        let mut real_room_name = String::new();
        for character in room_name.chars() {
          let mut next_character = character;
          for _ in 0..room_number {
            next_character = match next_character {
              '-' => '-',
              'z' => 'a',
              _ => char::from_u32((next_character as u32) + 1)
                .expect(format!("Invalid character '{}' in room name.", next_character).as_str()),
            };
          }
          real_room_name.push(next_character);
        }
        println!("Real: {}", real_room_name);

        if real_room_name.contains("north") {
          println!("Found North Pole objects: {}", room_number);
          return;
        }
      }
    },
  }
}
