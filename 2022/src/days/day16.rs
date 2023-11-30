use std::collections::BinaryHeap;

use crate::parser::{read_unsigned_int, skip_word};

const MAX_NEIGHBORS: usize = 8;
const ALPHABET_SIZE: usize = 26;
const MAX_VALVES: usize = ALPHABET_SIZE * ALPHABET_SIZE;
const MAX_TIME: u8 = 30;
const MAX_OPENED_VALVES: usize = (MAX_TIME / 2) as usize;

#[derive(Copy, Clone)]
struct Valve {
    neighbors: [u16; MAX_NEIGHBORS],
    amount_neighbors: u8,
    flow: u32,
}

impl Valve {
    fn new() -> Self {
        Valve {
            flow: 0,
            neighbors: [0; MAX_NEIGHBORS],
            amount_neighbors: 0,
        }
    }
    fn index_of(a: u8, b: u8) -> usize {
        (a - 'A' as u8) as usize * ALPHABET_SIZE + (b - 'A' as u8) as usize
    }
}

#[derive(Copy, Clone, Hash)]
enum Action {
    OpenValve,
    Move(u16),
}

#[derive(Copy, Clone, Hash)]
struct State {
    opened_valves: [u16; MAX_OPENED_VALVES],
    amount_opened_valves: u8,
    pressure_per_second: u16,
    released_pressure: u16,
    last_action: Option<Action>,
    position: u16,
    remaining_time: u8,
    cached_weight: i16,
}

impl State {
    fn weight(&self) -> i16 {
        // Tried different values before settling on this one
        // Lower = faster computation
        let expected_valve_flow_unlock_per_second = 8;
        let mut potential =
            (self.remaining_time as u16 / 2) * self.remaining_time as u16 * expected_valve_flow_unlock_per_second;
        if self.remaining_time % 2 == 1 {
            potential += (self.remaining_time as u16 - 1) * expected_valve_flow_unlock_per_second / 4;
        }
        (self.released_pressure + self.pressure_per_second * self.remaining_time as u16 + potential) as i16
    }

    fn nextState(n: u64) -> impl std::iter::Iterator<Item = State> {
        let mut D
        let mut num = 0;
        std::iter::from_fn(move || {
            let result;
            if num < n {
                result = Some(num);
                num += 1
            } else {
                result = None
            }
            result
        })
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.cached_weight == other.cached_weight
    }
}
impl Eq for State {}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cached_weight.cmp(&other.cached_weight)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub(crate) fn part1(buffer: &[u8]) -> String {
    let mut neighborhood = [Valve::new(); MAX_VALVES];

    let mut cursor: usize = 0;
    while cursor < buffer.len() {
        cursor += 6;
        let valve_index = Valve::index_of(buffer[cursor], buffer[cursor + 1]);
        cursor += 17;
        neighborhood[valve_index].flow = read_unsigned_int(buffer, &mut cursor).unwrap() as u32;
        cursor += 2;
        skip_word(buffer, &mut cursor); // tunnel.s
        cursor += 1;
        skip_word(buffer, &mut cursor); // lead.s
        cursor += 4;
        skip_word(buffer, &mut cursor); // valve.s
        cursor += 1;

        let mut has_other_valve = true;
        while has_other_valve {
            let neighbor_valve_index = Valve::index_of(buffer[cursor], buffer[cursor + 1]);
            neighborhood[valve_index].neighbors[neighborhood[valve_index].amount_neighbors as usize] =
                neighbor_valve_index as u16;
            neighborhood[valve_index].amount_neighbors += 1;

            if buffer[cursor + 2] != ',' as u8 {
                has_other_valve = false;
                cursor += 3
            } else {
                cursor += 4;
            }
        }
    }

    let mut potentials = BinaryHeap::<State>::new();
    let mut best_state = Some(State {
        opened_valves: [0; MAX_OPENED_VALVES],
        amount_opened_valves: 0,
        last_action: None,
        position: Valve::index_of('A' as u8, 'A' as u8) as u16,
        pressure_per_second: 0,
        released_pressure: 0,
        remaining_time: MAX_TIME,
        cached_weight: 0,
    });
    let mut n = 0;

    while match best_state.as_ref() {
        None => false,
        Some(state) => state.remaining_time > 0,
    } {
        n += 1;
        let mut current_state = best_state.unwrap();

        current_state.released_pressure += current_state.pressure_per_second;
        current_state.remaining_time -= 1;
        let current_valve = &neighborhood[current_state.position as usize];

        for i in 0..current_valve.amount_neighbors as usize {
            if match current_state.last_action.unwrap_or(Action::OpenValve) {
                Action::OpenValve => true,
                Action::Move(last_pos) => current_valve.neighbors[i] != last_pos,
            } {
                let mut new_state = current_state.clone();
                new_state.position = current_valve.neighbors[i];
                new_state.last_action = Some(Action::Move(current_state.position));
                new_state.cached_weight = new_state.weight();
                potentials.push(new_state);
            }
        }

        if match current_state.last_action.unwrap_or(Action::Move(0)) {
            Action::OpenValve => false,
            Action::Move(_) => true,
        } {
            let mut can_open_valve = current_valve.flow > 0;
            if can_open_valve {
                for i in 0..current_state.amount_opened_valves as usize {
                    if current_state.opened_valves[i] == current_state.position {
                        can_open_valve = false;
                        break;
                    }
                }
            }
            if can_open_valve {
                current_state.opened_valves[current_state.amount_opened_valves as usize] = current_state.position;
                current_state.amount_opened_valves += 1;
                current_state.last_action = Some(Action::OpenValve);
                current_state.pressure_per_second += current_valve.flow as u16;
                current_state.cached_weight = current_state.weight();
                potentials.push(current_state);
            }
        }

        best_state = potentials.pop();
    }

    return match best_state {
        None => "".to_string(),
        Some(state) => state.released_pressure.to_string(),
    };
}

pub(crate) fn part2(_buffer: &[u8]) -> String {
    return "".to_string();
}
