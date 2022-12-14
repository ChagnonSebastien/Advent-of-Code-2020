use std::alloc::{alloc, dealloc, Layout};
use std::cmp::Ordering;
use std::collections::HashSet;
use std::collections::binary_heap::BinaryHeap;
use std::marker::PhantomData;
use std::ptr::{NonNull, read, write};

const MAP_HEIGHT: usize = 41;
const MAP_WIDTH: usize = 179;
const MAP_AREA: usize = MAP_WIDTH * MAP_HEIGHT;

#[derive(Copy, Clone)]
struct Coordinate {
    top_neighbor: bool,
    left_neighbor: bool,
    right_neighbor: bool,
    bottom_neighbor: bool,
    height: u8,
}

fn sanitize_height(char: u8) -> u8 {
    match char as char {
        'S' => 'a' as u8,
        'E' => 'z' as u8,
        _any_other => char,
    }
}

trait Map {
    fn set_height(&mut self, offset: usize, coord: Coordinate);
    fn get_coordinate(&self, offset: usize) -> Coordinate;

    fn build_from_buffer(&mut self, buffer: &[u8]) -> (usize, usize) {
        let mut offset = 0;
        let (mut start, mut end) = (Default::default(), Default::default());
        for y in 0..MAP_HEIGHT as u8 {
            for x in 0..MAP_WIDTH as u8 {
                let parsed_offset = y as usize * MAP_WIDTH + x as usize;
                let mut height = buffer[offset];
                if height == 'S' as u8 {
                    start = parsed_offset;
                    height = 'a' as u8;
                } else if height == 'E' as u8 {
                    end = parsed_offset;
                    height = 'z' as u8;
                }

                let coord = Coordinate {
                    top_neighbor: match offset > MAP_WIDTH {
                        true => height - 1 <= sanitize_height(buffer[offset - (MAP_WIDTH + 1)]),
                        false => false,
                    },
                    left_neighbor: match offset % (MAP_WIDTH + 1) != 0 {
                        true => height - 1 <= sanitize_height(buffer[offset - 1]),
                        false => false,
                    },
                    right_neighbor: match (offset + 1) % (MAP_WIDTH + 1) != MAP_WIDTH {
                        true => height - 1 <= sanitize_height(buffer[offset + 1]),
                        false => false,
                    },
                    bottom_neighbor: match offset < (MAP_WIDTH + 1) * (MAP_HEIGHT - 1) {
                        true => height - 1 <= sanitize_height(buffer[offset + (MAP_WIDTH + 1)]),
                        false => false,
                    },
                    height,
                };

                self.set_height(parsed_offset, coord);
                offset += 1;
            }
            offset += 1;
        }
        return (start, end);
    }

    fn draw(&self, visited: &HashSet<usize>) {
        for offset in 0..MAP_AREA {
            if offset % MAP_WIDTH == 0 {
                println!()
            }
            if visited.contains(&offset) {
                print!("\x1b[92m");
            } else {
                print!("\x1b[0m");
            }
            print!("{}", self.get_coordinate(offset).height as char);
        }
        println!()
    }
}

struct MemMap {
    ptr: NonNull<Coordinate>,
    cap: usize,
    _marker: PhantomData<Coordinate>,
}

impl Drop for MemMap {
    fn drop(&mut self) {
        unsafe {
            dealloc(
                self.ptr.as_ptr() as *mut u8,
                Layout::array::<Coordinate>(self.cap).unwrap(),
            );
        }
    }
}

impl MemMap {
    fn new() -> Self {
        let cap = MAP_AREA;
        let layout = Layout::array::<Coordinate>(cap).unwrap();
        assert!(layout.size() <= isize::MAX as usize, "Allocation too large");

        MemMap {
            ptr: NonNull::new(unsafe { alloc(layout) } as *mut Coordinate).unwrap(),
            cap,
            _marker: PhantomData,
        }
    }
}

impl Map for MemMap {
    fn set_height(&mut self, offset: usize, coordinate: Coordinate) {
        unsafe {
            write(self.ptr.as_ptr().offset(offset as isize), coordinate);
        }
    }

    fn get_coordinate(&self, offset: usize) -> Coordinate {
        unsafe {
            read(self.ptr.as_ptr().offset(offset as isize))
        }
    }
}

struct StackMap {
    heightmap: [Coordinate; MAP_AREA],
}

impl StackMap {
    fn new() -> Self {
        StackMap {
            heightmap: [Coordinate {
                height: 0,
                top_neighbor: false,
                left_neighbor: false,
                right_neighbor: false,
                bottom_neighbor: false,
            }; MAP_AREA]
        }
    }
}

impl Map for StackMap {
    fn set_height(&mut self, offset: usize, coordinate: Coordinate) {
        self.heightmap[offset] = coordinate;
    }

    fn get_coordinate(&self, offset: usize) -> Coordinate {
        self.heightmap[offset]
    }
}

fn distance(pos: usize, goal: usize) -> usize {
    (pos / MAP_WIDTH).abs_diff(goal / MAP_WIDTH) + (pos % MAP_WIDTH).abs_diff(goal % MAP_WIDTH)
}

#[derive(Eq, Copy, Clone)]
struct Solution {
    moves: usize,
    pos: usize,
    distance: usize,
}

impl Solution {
    fn weight(&self) -> usize {
        (usize::MAX / 2) - (self.moves + self.distance)
    }
}

impl Ord for Solution {
    fn cmp(&self, other: &Self) -> Ordering {
        self.weight().cmp(&other.weight())
    }
}

impl PartialOrd<Self> for Solution {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.weight().partial_cmp(&other.weight())
    }
}

impl PartialEq<Self> for Solution {
    fn eq(&self, other: &Self) -> bool {
        self.weight().eq(&other.weight())
    }
}

pub(crate) fn part1(buffer: &[u8]) -> String {
    let mut map = StackMap::new();
    let (end, start) = map.build_from_buffer(buffer);

    let mut solutions = BinaryHeap::<Solution>::new();
    let mut visited = HashSet::<usize>::new();
    visited.insert(start);

    let mut considering = Some(Solution {
        moves: 0,
        pos: start,
        distance: distance(start, end),
    });

    while considering.is_some() {
        let current = considering.unwrap();
        if current.pos == end {
            break;
        }

        let coordinate = map.get_coordinate(current.pos);
        if coordinate.top_neighbor {
            let neighbor_pos = current.pos - MAP_WIDTH;
            if visited.insert(neighbor_pos) {
                solutions.push(Solution { moves: current.moves + 1, pos: neighbor_pos, distance: distance(neighbor_pos, end) })
            }
        }
        if coordinate.bottom_neighbor {
            let neighbor_pos = current.pos + MAP_WIDTH;
            if visited.insert(neighbor_pos) {
                solutions.push(Solution { moves: current.moves + 1, pos: neighbor_pos, distance: distance(neighbor_pos, end) })
            }
        }
        if coordinate.left_neighbor {
            let neighbor_pos = current.pos - 1;
            if visited.insert(neighbor_pos) {
                solutions.push(Solution { moves: current.moves + 1, pos: neighbor_pos, distance: distance(neighbor_pos, end) })
            }
        }
        if coordinate.right_neighbor {
            let neighbor_pos = current.pos + 1;
            if visited.insert(neighbor_pos) {
                solutions.push(Solution { moves: current.moves + 1, pos: neighbor_pos, distance: distance(neighbor_pos, end) })
            }
        }

        considering = solutions.pop();
    }

    // map.draw(&visited);
    return considering.unwrap().moves.to_string()
}

pub(crate) fn part2(buffer: &[u8]) -> String {
    let mut map = MemMap::new();
    let (_, end) = map.build_from_buffer(buffer);

    let mut solutions = BinaryHeap::<Solution>::new();
    let mut visited = HashSet::<usize>::new();
    visited.insert(end);

    let mut considering = Some(Solution {
        moves: 0,
        pos: end,
        distance: 0,
    });

    while considering.is_some() {
        let current = considering.unwrap();
        let coordinate = map.get_coordinate(current.pos);
        if coordinate.height == 'a' as u8 {
            break;
        }

        if coordinate.top_neighbor {
            let neighbor_pos = current.pos - MAP_WIDTH;
            if visited.insert(neighbor_pos) {
                solutions.push(Solution { moves: current.moves + 1, pos: neighbor_pos, distance: neighbor_pos % MAP_WIDTH })
            }
        }
        if coordinate.bottom_neighbor {
            let neighbor_pos = current.pos + MAP_WIDTH;
            if visited.insert(neighbor_pos) {
                solutions.push(Solution { moves: current.moves + 1, pos: neighbor_pos, distance: neighbor_pos % MAP_WIDTH })
            }
        }
        if coordinate.left_neighbor {
            let neighbor_pos = current.pos - 1;
            if visited.insert(neighbor_pos) {
                solutions.push(Solution { moves: current.moves + 1, pos: neighbor_pos, distance: neighbor_pos % MAP_WIDTH })
            }
        }
        if coordinate.right_neighbor {
            let neighbor_pos = current.pos + 1;
            if visited.insert(neighbor_pos) {
                solutions.push(Solution { moves: current.moves + 1, pos: neighbor_pos, distance: neighbor_pos % MAP_WIDTH })
            }
        }

        considering = solutions.pop();
    }

    // map.draw(&visited);
    return considering.unwrap().moves.to_string()
}
