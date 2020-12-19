use crate::utils::Part;

pub fn execute(input: String, part: &Part) {
  let disk_size = match part { Part::PartOne => 272, Part::PartTwo => 35651584 };
  let mut data = String::from(input);
  while data.len() < disk_size {
    
    data = format!("{}0{}", data, data.chars().rev().map(|x| match x {
      '0' => '1',
      '1' => '0',
      _ => panic!("Invalid char: {}", x),
    }).collect::<String>());
  }

  let final_data = &data[0..disk_size];
  let mut checksum = String::from(final_data);
  while checksum.len() % 2 == 0 {
    let chars = checksum.chars().collect::<Vec<char>>();
    checksum = String::new();
    for i in 0..chars.len()/2 {
      checksum.push(match chars[2*i] == chars[2*i+1] { true => '1', false => '0' });
    }
  }

  println!("Checksum: {}", checksum);
}