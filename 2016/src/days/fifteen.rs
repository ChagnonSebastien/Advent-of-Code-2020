use crate::utils::Part;
use regex::Regex;

struct Disk {
  offset: usize,
  size: usize,
  position: usize,
}

impl Disk {
  fn from_string(s: &str) -> Self {
    let re = Regex::new(r"[0-9]+").unwrap();
    let parts = re.find_iter(s).map(|x| x.as_str().parse::<usize>().unwrap()).collect::<Vec<usize>>();

    Disk {
      offset: parts[0],
      size: parts[1],
      position: parts[3],
    }
  }

  fn is_aligned(&self, time: usize) -> bool {
    (self.offset + self.position + time) % self.size == 0
  }
}

fn is_solution(time: usize, disks: &Vec<Disk>) -> bool {
  for disk in disks {
    if !disk.is_aligned(time) {
      return false;
    }
  }
  true
}

pub fn execute(input: String, part: &Part) {
  let mut disks = Vec::<Disk>::new();
  for line in input.split("\n") {
    disks.push(Disk::from_string(line));
  }

  if *part == Part::PartTwo {
    disks.push(Disk {
      offset: disks.len() + 1,
      size: 11,
      position: 0,
    });
  }

  let mut i = 0;
  while !is_solution(i, &disks) {
    i += 1;
  }
  println!("Must push button at second {}.", i);
}