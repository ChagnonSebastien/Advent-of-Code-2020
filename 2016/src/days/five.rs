use crate::utils::Part;
use md5;

fn is_password_filled(password: [Option<char>; 8]) -> bool {
  for character in &password {
    if character.is_none() {
      return false;
    }
  }
  return true;
}

pub fn execute(input: String, part: &Part) {
  match part {
    Part::PartOne => {
      let mut password = String::new();
      let mut i: u64 = 0;
      while password.len() < 8 {
        let digest = md5::compute(format!("{}{}", input, i));
        let hash: String = format!("{:?}", digest);
        if &hash[..5] == "00000" {
          println!("Found match : Hash({}{}) == {}", input, i, hash);
          password.push(hash.chars().nth(5).expect("The hash has less than 6 characters"));
        }
        i += 1;
      }
    
      println!("\nThe password is: {}", password);
    },
    Part::PartTwo => {
      let mut password: [Option<char>; 8] = [None; 8];
      let mut i: u64 = 0;
      while !is_password_filled(password) {
        let digest = md5::compute(format!("{}{}", input, i));
        let hash: String = format!("{:?}", digest);
        if &hash[..5] == "00000" {
          println!("Found match : Hash({}{}) == {}", input, i, hash);
          let index = hash.chars().nth(5).unwrap_or('8').to_digit(8).unwrap_or(8);
          if index >= 8 {
            println!("Invalid index value.");
          } else if password[index as usize].is_some() {
            println!("Index already filled.");
          } else {
            password[index as usize] = hash.chars().nth(6);
          }
        }
        i += 1;
      }
    
      let password_reduce = password.iter().map(|c| c.expect("Invalid password character"));
      println!("\nThe password is: {}", password_reduce.collect::<String>());
    },
  };
}
