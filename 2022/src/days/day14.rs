use std::collections::HashSet;
use crate::parser::read_unsigned_int;
use crate::vector::Vector2D;

const DOWN: Vector2D<i16> = Vector2D { x: 0, y: 1 };
const DOWN_LEFT: Vector2D<i16> = Vector2D { x: -1, y: 1 };
const DOWN_RIGHT: Vector2D<i16> = Vector2D { x: 1, y: 1 };

const SAND_SOURCE: Vector2D<i16> = Vector2D { x: 500, y: 0 };

trait MapState {
    fn is_solid(&self, position: &Vector2D<i16>) -> bool;
    fn set_solid(&mut self, position: Vector2D<i16>);
}

struct HashSetMap {
    solid: HashSet<Vector2D<i16>>,
}

impl MapState for HashSetMap {
    fn is_solid(&self, position: &Vector2D<i16>) -> bool {
        self.solid.contains(&position)
    }

    fn set_solid(&mut self, position: Vector2D<i16>) {
        self.solid.insert(position);
    }
}

const MAX_HEIGHT: usize = 170;
const MIN_X: usize = 500 - MAX_HEIGHT;
const AMOUNT_WIDTH_SECTIONS: usize = ((MAX_HEIGHT * 2) / 64) + 1;

struct StackMap {
    solid: [[u64; MAX_HEIGHT]; AMOUNT_WIDTH_SECTIONS]
}

impl MapState for StackMap {
    fn is_solid(&self, position: &Vector2D<i16>) -> bool {
        let adjusted_x = position.x as usize - MIN_X;
        let segment = adjusted_x / 64;
        let mask = 1 << (adjusted_x % 64);
        self.solid[segment][position.y as usize] & mask > 0
    }

    fn set_solid(&mut self, position: Vector2D<i16>) {
        let adjusted_x = position.x as usize - MIN_X;
        let segment = adjusted_x / 64;
        let mask = 1 << (adjusted_x % 64);
        self.solid[segment][position.y as usize] |= mask;
    }
}

fn parse_vector2d(buffer: &[u8], offset: &mut usize) -> Vector2D<i16> {
    let x = read_unsigned_int(buffer, offset).unwrap() as i16;
    *offset += 1;
    let y = read_unsigned_int(buffer, offset).unwrap() as i16;
    Vector2D { x, y }
}

fn unit(vector: Vector2D<i16>) -> Vector2D<i16> {
    if vector.x == 0 {
        if vector.y > 0 {
            Vector2D { x: 0, y: 1 }
        } else {
            Vector2D { x: 0, y: -1 }
        }
    } else if vector.y == 0 {
        if vector.x > 0 {
            Vector2D { x: 1, y: 0 }
        } else {
            Vector2D { x: -1, y: 0 }
        }
    } else {
        unimplemented!()
    }
}

fn simulate_simple(position: Vector2D<i16>, lowest_point: i16, solid: &mut dyn MapState, stop_at_fallthrough: bool) -> (usize, bool) {
    if stop_at_fallthrough && position.y > lowest_point {
            return (0, true);
    }

    if solid.is_solid(&position) {
        return (0, false);
    }

    if !stop_at_fallthrough && position.y == lowest_point + 1 {
        solid.set_solid(position);
        return (1, false);
    }

    let mut solidified = 0;
    for falling_direction in [DOWN, DOWN_LEFT, DOWN_RIGHT] {
        let (new_solidified, fell_through) = simulate_simple(position + falling_direction, lowest_point, solid, stop_at_fallthrough);
        solidified += new_solidified;
        if fell_through {
            return (solidified, true);
        }
    }

    solid.set_solid(position);
    return (solidified + 1, false);
}

fn simulate_complex(position: Vector2D<i16>, lowest_point: i16, solid: &mut dyn MapState, stop_at_fallthrough: bool, remaining_only_left_for: usize, remaining_only_right_for: usize) -> (usize, bool, usize, usize) {
    if stop_at_fallthrough && position.y > lowest_point {
        return (0, true, 0, 0);
    }

    if solid.is_solid(&position) {
        return (0, false, 0, 0);
    }

    if !stop_at_fallthrough && position.y == lowest_point + 1 {
        solid.set_solid(position);
        return (1, false, 0, 0);
    }

    let mut solidified = 0;
    let mut only_left_for = 0;
    let mut only_right_for = 0;
    if remaining_only_left_for == 0 && remaining_only_right_for == 0 {
        let (new_solidified, fell_through, new_only_left_for, new_only_right_for) = simulate_complex(position + DOWN, lowest_point, solid, stop_at_fallthrough, 0, 0);
        solidified += new_solidified;
        if fell_through {
            return (solidified, true, 0, 0);
        }
        only_left_for = new_only_left_for;
        only_right_for = new_only_right_for;
    }
    if remaining_only_right_for == 0 {
        if only_left_for > 0 {
            only_left_for -= 1;
        }
        let (new_solidified, fell_through, new_only_left_for, _) = simulate_complex(position + DOWN_LEFT, lowest_point, solid, stop_at_fallthrough, only_left_for, 0);
        solidified += new_solidified;
        if fell_through {
            return (solidified, true, 0, 0);
        }
        only_left_for = new_only_left_for + 1;
    }
    if remaining_only_left_for == 0 {
        if only_right_for > 0 {
            only_right_for -= 1;
        }
        let (new_solidified, fell_through, _, new_only_right_for) = simulate_complex(position + DOWN_RIGHT, lowest_point, solid, stop_at_fallthrough, 0, only_right_for);
        solidified += new_solidified;
        if fell_through {
            return (solidified, true, 0, 0);
        }
        only_right_for = new_only_right_for + 1;
    }

    solid.set_solid(position);
    return (solidified + 1, false, only_left_for, only_right_for);
}

fn load_structures(buffer: &[u8], state: &mut dyn MapState) -> i16 {
    let mut offset = 0;
    let mut lowest_point = 0;

    while offset < buffer.len() {
        let mut from = parse_vector2d(buffer, &mut offset);
        state.set_solid(from);

        while buffer[offset] != '\n' as u8 {
            offset += 4;
            let to = parse_vector2d(buffer, &mut offset);
            if to.y > lowest_point {
                lowest_point = to.y;
            }

            let step = unit(to - from);

            while from != to {
                from += step;
                state.set_solid(from);
            }
        }
        offset += 1;
    }
    return lowest_point;
}

pub(crate) fn part1_simple(buffer: &[u8]) -> String {
    let mut state = HashSetMap { solid: HashSet::new() };
    let lowest_point = load_structures(buffer, &mut state);
    let (solidified, _) = simulate_simple(SAND_SOURCE, lowest_point, &mut state, true);
    return solidified.to_string()
}

pub(crate) fn part2_simple(buffer: &[u8]) -> String {
    let mut state = HashSetMap { solid: HashSet::new() };
    let lowest_point = load_structures(buffer, &mut state);
    let (solidified, _) = simulate_simple(SAND_SOURCE, lowest_point, &mut state, false);
    return solidified.to_string()
}

pub(crate) fn part1_complex_hashset(buffer: &[u8]) -> String {
    let mut state = HashSetMap { solid: HashSet::new() };
    let lowest_point = load_structures(buffer, &mut state);
    let (solidified, _, _, _) = simulate_complex(SAND_SOURCE, lowest_point, &mut state, true, 0, 0);
    return solidified.to_string()
}

pub(crate) fn part2_complex_hashset(buffer: &[u8]) -> String {
    let mut state = HashSetMap { solid: HashSet::new() };
    let lowest_point = load_structures(buffer, &mut state);
    let (solidified, _, _, _) = simulate_complex(SAND_SOURCE, lowest_point, &mut state, false, 0, 0);
    return solidified.to_string()
}

pub(crate) fn part1(buffer: &[u8]) -> String {
    let mut state = StackMap { solid: [[0; MAX_HEIGHT]; AMOUNT_WIDTH_SECTIONS] };
    let lowest_point = load_structures(buffer, &mut state);
    let (solidified, _, _, _) = simulate_complex(SAND_SOURCE, lowest_point, &mut state, true, 0, 0);
    return solidified.to_string()
}

pub(crate) fn part2(buffer: &[u8]) -> String {
    let mut state = StackMap { solid: [[0; MAX_HEIGHT]; AMOUNT_WIDTH_SECTIONS] };
    let lowest_point = load_structures(buffer, &mut state);
    let (solidified, _, _, _) = simulate_complex(SAND_SOURCE, lowest_point, &mut state, false, 0, 0);
    return solidified.to_string()
}