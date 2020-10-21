use crate::utils::Part;
use regex::Regex;

fn decompress(input: &str, recursive: bool) -> String {
  let re = Regex::new(r"\([0-9]+x[0-9]+\)").unwrap();
  let mut output = String::new();
  let mut position = 0;

  for marker in re.find_iter(input) {
    if marker.start() < position { continue }

    output.push_str(&input[position..marker.start()]);

    let tag = marker.as_str();
    let parts = String::from(tag)[1..tag.len()-1]
      .split("x")
      .map(|x| x.parse::<usize>().unwrap())
      .collect::<Vec<usize>>();

    let raw_pattern = &input[marker.end()..marker.end()+parts[0]];
    let pattern = match recursive {
      false => String::from(raw_pattern),
      true => decompress(raw_pattern, true),
    };
    for _ in 0..parts[1] {
      output.push_str(pattern.as_str());
    }
    position = marker.end()+parts[0];
  }
  output.push_str(&input[position..input.len()]);
  output
}

pub fn execute(input: String, part: &Part) {
  let output = decompress(input.as_str(), *part == Part::PartTwo);
  println!("Decompressed length:  {}", output.len());
}
