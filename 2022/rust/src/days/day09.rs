use std::collections::HashSet;
use std::ops::AddAssign;
use crate::parser::read_unsigned_int;

enum Direction {
    LEFT, RIGHT, UP, DOWN
}

struct Instruction {
    direction: Direction,
    distance: usize,
}

impl Instruction {
    fn parse(buffer: &[u8], offset: &mut usize) -> Self {
        let char = buffer[*offset] as char;
        *offset += 2;
        let amount = read_unsigned_int(buffer, offset).unwrap();
        *offset += 1;

        match char {
            'L' => Instruction { direction: Direction::LEFT, distance: amount },
            'R' => Instruction { direction: Direction::RIGHT, distance: amount },
            'D' => Instruction { direction: Direction::DOWN, distance: amount },
            'U' => Instruction { direction: Direction::UP, distance: amount },
            _ => panic!("Unknown Direction: {}", char),
        }
    }
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Point {
    x: i16,
    y: i16,
}

impl AddAssign<Point> for Point {
    fn add_assign(&mut self, rhs: Point) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

pub(crate) fn part1(input: &String) -> String {
    let buffer = input.as_bytes();
    let mut offset = 0;

    let mut head = Point { x: 0, y: 0 };
    let mut tail = Point { x: 0, y: 0 };

    let mut visited = HashSet::<Point>::with_capacity(input.len() / 4);
    visited.insert(tail);

    while offset < buffer.len() {
        let instruction = Instruction::parse(buffer, &mut offset);
        match instruction.direction {
            Direction::LEFT => {
                for _ in 0..instruction.distance {
                    head.x -= 1;
                    if tail.x - head.x > 1 {
                        tail.x -= 1;
                        tail.y = head.y;
                        visited.insert(tail);
                    }
                }
            }
            Direction::RIGHT => {
                for _ in 0..instruction.distance {
                    head.x += 1;
                    if head.x - tail.x > 1 {
                        tail.x += 1;
                        tail.y = head.y;
                        visited.insert(tail);
                    }
                }
            }
            Direction::DOWN => {
                for _ in 0..instruction.distance {
                    head.y -= 1;
                    if tail.y - head.y > 1 {
                        tail.y -= 1;
                        tail.x = head.x;
                        visited.insert(tail);
                    }
                }
            }
            Direction::UP => {
                for _ in 0..instruction.distance {
                    head.y += 1;
                    if head.y - tail.y > 1 {
                        tail.y += 1;
                        tail.x = head.x;
                        visited.insert(tail);
                    }
                }
            }
        }
    }

    return visited.len().to_string()
}

pub(crate) fn part2(input: &String) -> String {
    let buffer = input.as_bytes();
    let mut offset = 0;

    let mut rope = [Point { x: 0, y: 0 }; 10];

    let mut visited = HashSet::<Point>::with_capacity(input.len() / 4);
    visited.insert(Point { x: 0, y: 0 });

    while offset < buffer.len() {
        let instruction = Instruction::parse(buffer, &mut offset);
        for _ in 0..instruction.distance {
            let mut shift = match instruction.direction {
                Direction::LEFT => Point { x: -1, y: 0 },
                Direction::RIGHT => Point { x: 1, y: 0 },
                Direction::UP => Point { x: 0, y: 1 },
                Direction::DOWN => Point { x: 0, y: -1 },
            };
            let mut knot = 0;
            while knot < 10 {
                rope[knot] += shift;

                if knot == 9 {
                    visited.insert(rope[knot]);
                } else {
                    let diff_x = (rope[knot].x - rope[knot + 1].x).abs();
                    let diff_y = (rope[knot].y - rope[knot + 1].y).abs();

                    if diff_x > 1 || diff_y > 1 {
                        match diff_x {
                            0 => shift.x = 0,
                            1 => shift.x = rope[knot].x - rope[knot + 1].x,
                            2 => shift.x = (rope[knot].x - rope[knot + 1].x) / 2,
                            _ => panic!("How is there a more than 2 difference, this shouldn't happen..")
                        }
                        match diff_y {
                            0 => shift.y = 0,
                            1 => shift.y = rope[knot].y - rope[knot + 1].y,
                            2 => shift.y = (rope[knot].y - rope[knot + 1].y) / 2,
                            _ => panic!("How is there a more than 2 difference, this shouldn't happen..")
                        }
                    } else {
                        break;
                    }
                }

                knot += 1;
            }
        }
    }

    return visited.len().to_string()
}