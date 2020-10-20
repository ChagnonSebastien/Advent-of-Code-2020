use crate::utils::Part;
use regex;

fn contains_abba(section: &str) -> bool {
  for i in 0..section.len() - 3 {
    let segment = &section[i..i+4];
    if
      segment.chars().nth(0).unwrap() == segment.chars().nth(3).unwrap() &&
      segment.chars().nth(1).unwrap() == segment.chars().nth(2).unwrap() &&
      segment.chars().nth(0).unwrap() != segment.chars().nth(1).unwrap()
    {
      return true
    }
  }
  false
}

fn get_all_aba(section: &str) -> Vec<String> {
  let mut aba_list: Vec<String> = Vec::new();
  for i in 0..section.len() - 2 {
    let segment = &section[i..i+3];
    if
      segment.chars().nth(0).unwrap() == segment.chars().nth(2).unwrap() &&
      segment.chars().nth(0).unwrap() != segment.chars().nth(1).unwrap()
    {
      aba_list.push(String::from(segment));
    }
  }
  aba_list
}

fn valid_ip(ip: &str) -> bool {
  let re = regex::Regex::new(r"\[|\]").unwrap();
  let mut inside_square_brackets = false;
  let mut has_abba = false;
  let mut invalid = false;
  for section in re.split(ip) {
    if contains_abba(section) {
      match inside_square_brackets {
        true => invalid = true,
        false => has_abba = true,
      }
    }

    inside_square_brackets = !inside_square_brackets;
  }

  has_abba && !invalid
}

fn supports_ssl(ip: &str) -> bool {
  let re = regex::Regex::new(r"\[|\]").unwrap();
  let mut inside_square_brackets = false;
  let mut aba_list: Vec<String> = Vec::new();
  let mut bab_list: Vec<String> = Vec::new();
  for section in re.split(ip) {
    let mut finds: Vec<String> = get_all_aba(section);
    match inside_square_brackets {
      true => bab_list.append(&mut finds),
      false => aba_list.append(&mut finds),
    }

    inside_square_brackets = !inside_square_brackets;
  }

  for aba in &aba_list {
    let mut pattern_aba = aba.chars();
    let a = pattern_aba.next();
    let b = pattern_aba.next();
    for bab in &bab_list {
      let mut pattern_bab = bab.chars();
      if pattern_bab.next() == b && pattern_bab.next() == a {
        return true
      }
    }
  }

  false
}

pub fn execute(input: String, part: &Part) {
  let mut valid_ips_count = 0;
  for ip in input.split("\n") {
    if match part {
      Part::PartOne => valid_ip(ip),
      Part::PartTwo => supports_ssl(ip),
    } {
      println!("Valid IP: \n{}", ip);
      valid_ips_count += 1;
    }
  }

  println!("\nAmount of valid IPs: \n{}", valid_ips_count);
}
