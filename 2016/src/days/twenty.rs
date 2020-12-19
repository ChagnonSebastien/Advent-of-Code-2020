use crate::utils::Part;

#[derive(Copy, Clone)]
struct Blacklist {
  from: u32,
  to: u32,
}

#[derive(Copy, Clone)]
struct Whitelist {
  from: u32,
  to: u32,
}

impl Whitelist {
  fn intersects(&self, bl: &Blacklist) -> bool {
    self.from <= bl.to && self.to >= bl.from
  }

  fn intersection(&self, bl: &Blacklist) -> Vec<Self> {
    let mut intersection = Vec::<Self>::new();

    if self.intersects(bl) {
      if bl.from > self.from {
        intersection.push(Self { from: self.from, to: bl.from - 1 });
      }

      if bl.to < self.to {
        intersection.push(Self { from: bl.to + 1, to: self.to });
      }
    } else {
      intersection.push(Self { from: self.from, to: self.to });
    }

    intersection
  }
}

pub fn execute(input: String, part: &Part) {
  let blacklists = input.split("\n").map(|range| {
    let parts = range.split("-").map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
    Blacklist { from: parts[0], to: parts[1] }
  }).collect::<Vec<Blacklist>>();

  let mut whitelists = Vec::<Whitelist>::new();
  whitelists.push(Whitelist { from: 0, to: 4_294_967_295});

  for bl in blacklists {
    let mut i = 0;
    while i < whitelists.len() {
      let wl = *whitelists.get(i).unwrap();
      if wl.intersects(&bl) {
        whitelists.remove(i);
        whitelists.append(&mut wl.intersection(&bl));
      } else {
        i += 1;
      }
    }
  }


  match part {
    Part::PartOne => {
      let mut smallest_available_ip = 4_294_967_295;
      for wl in &whitelists {
        if wl.from < smallest_available_ip {
          smallest_available_ip = wl.from;
        }
      }
      println!("Smallest IP: {}", smallest_available_ip);
    },
    Part::PartTwo => {
      let mut total_available = 0;
      for wl in &whitelists {
        total_available += 1 + wl.to - wl.from;
      }
      println!("Total Ips: {}", total_available);
    }
  }
}
