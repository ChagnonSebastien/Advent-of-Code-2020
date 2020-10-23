use crate::utils::Part;
use regex::Regex;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Copy, Clone)]
enum Element {
  Thulium, Plutonium, Strontium, Promethium, Ruthenium
}

impl Element {
  fn from_string(name: &str) -> Element {
    match name {
      "thulium" => Element::Thulium,
      "plutonium" => Element::Plutonium,
      "strontium" => Element::Strontium,
      "promethium" => Element::Promethium,
      "ruthenium" => Element::Ruthenium,
      _ => panic!("Invalid Element Name: {}", name),
    }
  }
}

impl Display for Element {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    write!(f, "{}", match self {
      Element::Thulium => "Thulium",
      Element::Plutonium => "Plutonium",
      Element::Strontium => "Strontium",
      Element::Promethium => "Promethium",
      Element::Ruthenium => "Ruthenium",
    })
  }
}

#[derive(Copy, Clone)]
enum Equipment {
  Microchip(Element), Generator(Element)
}

#[derive(Copy, Clone)]
struct Floor {
  generators: [bool; 5],
  microships: [bool; 5],
}

impl Display for Floor {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    let mut floor_representation = String::new();
    for i in 0..5 {
      let potential_microship = format!(" M.{} ", i);
      let potential_generator = format!(" G.{} ", i);
      floor_representation.push_str(match self.microships[i] { true => potential_microship.as_str(), false => "     " });
      floor_representation.push_str(match self.generators[i] { true => potential_generator.as_str(), false => "     " });
    }
    write!(f, "{}", floor_representation)
  }
}

impl Floor {
  fn is_valid(&self) -> bool {
    !self.has_unpluged_microchip() || !self.has_at_least_one_generator()
  }

  fn has_unpluged_microchip(&self) -> bool {
    for i in 0..5 {
      if self.microships[i] && !self.generators[i] {
        return true;
      }
    }
    false
  }

  fn generator_amount(&self) -> usize {
    self.generators.iter().fold(0, |sum, g| sum + match g { true => 1, false => 0 })
  }

  fn microship_amount(&self) -> usize {
    self.microships.iter().fold(0, |sum, g| sum + match g { true => 1, false => 0 })
  }

  fn amount_equipment(&self) -> usize {
    self.generator_amount() + self.microship_amount()
  }

  fn has_at_least_one_generator(&self) -> bool {
    self.generator_amount() > 0
  }

  fn install_equipment(&mut self, equipment: Equipment) {
    match equipment {
      Equipment::Generator(element) => self.generators[Self::element_index(element)] = true,
      Equipment::Microchip(element) => self.microships[Self::element_index(element)] = true,
    }
  }

  fn element_index(element: Element) -> usize {
    match element {
      Element::Thulium => 0,
      Element::Plutonium => 1,
      Element::Strontium => 2,
      Element::Promethium => 3,
      Element::Ruthenium => 4,
    }
  }

  fn empty() -> Self {
    Floor {
      generators: [false; 5],
      microships: [false; 5],
    }
  }
}

impl PartialEq for Floor {
    fn eq(&self, other: &Self) -> bool {
        self.generators == other.generators && self.microships == other.microships
    }
}
impl Eq for Floor {}

#[derive(Copy, Clone)]
struct Building {
  floors: [Floor; 4],
  moves: usize,
  elevator: usize,
}

impl Display for Building {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {

    let mut building_representation = String::from("=======================================================\n");
    for i in 0..4 {
      building_representation.push_str(format!("|{} {} |\n", match 3-i == self.elevator {
        true => "E",
        false => " ",
      }, self.floors[3-i]).as_str());
    }
    building_representation.push_str("=======================================================");
    write!(f, "{}", building_representation)
  }
}

impl Building {
  fn possible_moves(&self) -> Vec<Self> {
    let mut available_equipment: Vec<(bool, usize)> = Vec::new();
    for i in 0..5 {
      if self.floors[self.elevator].generators[i] {
        available_equipment.push((false, i));
      }
      if self.floors[self.elevator].microships[i] {
        available_equipment.push((true, i));
      }
    }

    let mut possibilities: Vec<Self> = Vec::new();
    let prev_floor = self.elevator;
    for next_floor in self.next_floors() {
      for i in 0..available_equipment.len() {
        let mut simple_neighbor = *self;
        simple_neighbor.moves += 1;
        simple_neighbor.elevator = next_floor;

        let simple = available_equipment[i];
        match simple.0 {
          true => {
            simple_neighbor.floors[prev_floor].microships[simple.1] = false;
            simple_neighbor.floors[next_floor].microships[simple.1] = true;
          },
          false => {
            simple_neighbor.floors[prev_floor].generators[simple.1] = false;
            simple_neighbor.floors[next_floor].generators[simple.1] = true;
          },
        }
        if simple_neighbor.is_valid() {
          possibilities.push(simple_neighbor);
        }

        for j in (i + 1)..available_equipment.len() {
          let mut double_neighbor = simple_neighbor;
          let double = available_equipment[j];
          match double.0 {
            true => {
              double_neighbor.floors[prev_floor].microships[double.1] = false;
              double_neighbor.floors[next_floor].microships[double.1] = true;
            },
            false => {
              double_neighbor.floors[prev_floor].generators[double.1] = false;
              double_neighbor.floors[next_floor].generators[double.1] = true;
            },
          }
          if double_neighbor.is_valid() {
            possibilities.push(double_neighbor);
          }
        }
      }
    }
    possibilities
  }

  fn is_valid(&self) -> bool {
    for floor in self.floors.iter() {
      if !floor.is_valid() {
        return false;
      }
    }
    true
  }

  fn install_equipment(&mut self, equipment: Equipment, floor_number: usize) {
    self.floors[floor_number].install_equipment(equipment);
  }

  fn cost(&self) -> usize {
    let mut cost = self.moves * 5;
    for i in 0..4 {
      cost += self.floors[i].amount_equipment() * (3-i);
    }
    cost
  }

  fn next_floors(&self) -> Vec<usize> {
    let mut next = Vec::new();
    if self.elevator < 3 {
      next.push(self.elevator + 1);
    }
    if self.elevator > 0 {
      next.push(self.elevator - 1);
    }
    next
  }

  fn has_same_floors(&self, other: &Self) -> bool {
    self.floors == other.floors && self.elevator == other.elevator
  }

  fn is_solution(&self) -> bool {
    self.floors[3].amount_equipment() == 10
  }

  fn empty() -> Self {
    Building {
      floors: [Floor::empty(), Floor::empty(), Floor::empty(), Floor::empty()],
      moves: 0,
      elevator: 0,
    }
  }
}

impl Ord for Building {
  fn cmp(&self, other: &Self) -> Ordering {
    other.cost().cmp(&self.cost())
  }
}

impl PartialOrd for Building {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Building {
    fn eq(&self, other: &Self) -> bool {
        self.cost() == other.cost()
    }
}
impl Eq for Building {}

pub fn execute(input: String, part: &Part) {
  let mut initial_building = Building::empty();
  let floor_contents = input.split("\n").collect::<Vec<&str>>();

  let re_generator = Regex::new(r"[a-z]+ generator").unwrap();
  for i in 0..floor_contents.len() {
    for element_match in re_generator.find_iter(floor_contents[i]) {
      let element_name = element_match.as_str().split(" ").next().unwrap();
      let element = Element::from_string(element_name);
      initial_building.install_equipment(Equipment::Generator(element), i);
    }
  }

  let re_microship = Regex::new(r"[a-z]+-compatible microchip").unwrap();
  for i in 0..floor_contents.len() {
    for element_match in re_microship.find_iter(floor_contents[i]) {
      let element_name = element_match.as_str().split("-").next().unwrap();
      let element = Element::from_string(element_name);
      initial_building.install_equipment(Equipment::Microchip(element), i);
    }
  }

  let mut visited: Vec<Building> = Vec::new();
  let mut solutions: BinaryHeap<Building> = BinaryHeap::new();
  solutions.push(initial_building);

  while {
    let possibility = solutions.pop().expect("No possibilities left");
    match possibility.is_solution() {
      true => {
        println!("Found Solution! Moves: {}", possibility.moves);
        false
      },
      false => {
        visited.push(possibility);
        for next_possibility in possibility.possible_moves() {
          if solutions.iter().find_map(|s| match s.has_same_floors(&next_possibility) {
            true => Some(s),
            false => None
          }).is_none() && visited.iter().find_map(|s| match s.has_same_floors(&next_possibility) {
            true => Some(s),
            false => None
          }).is_none(){
            solutions.push(next_possibility);
          }
        }
        true
      },
    }
  } {}
}
